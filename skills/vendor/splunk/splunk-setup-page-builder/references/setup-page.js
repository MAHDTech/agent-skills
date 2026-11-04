"use strict";

/*
 * Setup page logic for splunk_setup_demo.
 *
 * Based on Splunk's certification-standard setup-page examples
 * (splunk/splunk-app-examples/setup_pages/{setup_page_simple,developer_guidance_setup_page}),
 * with one deliberate deviation from those examples (see note below):
 *
 *   1. Validate the form inputs client-side.
 *   2. Write the non-secret setting into a custom .conf file via the
 *      `configurations` REST collection (create the file/stanza if they
 *      don't exist yet, then update the stanza's properties).
 *   3. Write the secret into storage/passwords (create if it doesn't
 *      exist for this realm+name, otherwise update it).
 *   4. Flip app.conf [install] is_configured to 1.
 *   5. Reload the app so splunkd picks up the on-disk changes, then
 *      redirect to the app's home page.
 *
 * This intentionally always performs steps 2-5 on submit, even if
 * is_configured is already 1. Splunk's own reference examples skip
 * straight to reload+redirect when already configured, which silently
 * discards any new values a user enters when reopening this page later
 * (e.g. via Manage Apps > Set up) to rotate a credential or change a
 * setting. That is a real, reproducible bug we hit and fixed while
 * proving out this skill: DO NOT reintroduce an is_configured guard here.
 *
 * Deliberately does not use default/setup.xml (deprecated, unsupported on
 * Splunk Cloud / SHC, fails AppInspect's cloud check).
 */

const APP_NAME = "splunk_setup_demo";
const APP_NAMESPACE = {
    owner: "nobody",
    app: APP_NAME,
    sharing: "app",
};

const SETTINGS_CONF_FILE = "splunk_setup_demo_settings";
const SETTINGS_STANZA = "endpoint";

const PASSWORD_REALM = "splunk_setup_demo_realm";
const PASSWORD_NAME = "api_token";

require(["jquery", "splunkjs/splunk"], function ($, splunkjs) {
    $("#setup_button").click(completeSetup);

    async function completeSetup() {
        clearMessages();

        const endpointUrl = $("#endpoint_url_input").val().trim();
        const apiToken = $("#api_token_input").val().trim();

        const validationErrors = validate(endpointUrl, apiToken);
        if (validationErrors.length > 0) {
            showErrors(validationErrors);
            return;
        }

        let stage = "Initializing the Splunk SDK for JavaScript";
        try {
            const http = new splunkjs.SplunkWebHttp();
            const service = new splunkjs.Service(http, APP_NAMESPACE);

            // Note: we deliberately do NOT skip saving just because
            // is_configured is already 1. A user can reach this page
            // after first-run setup (e.g. via Manage Apps > Set up) to
            // rotate the API token or change the endpoint URL, and that
            // resubmission must actually be persisted, not silently
            // discarded. Splunk's own auto-redirect-to-setup-page
            // behavior (driven by is_configured) already prevents this
            // page from being forced on users who are already configured;
            // it does not need to be re-enforced here on submit.

            stage = `Saving ${SETTINGS_STANZA}/endpoint_url to ${SETTINGS_CONF_FILE}.conf`;
            await saveConfigSetting(service, { endpoint_url: endpointUrl });

            stage = `Saving the API token to storage/passwords (realm=${PASSWORD_REALM})`;
            await saveSecret(service, apiToken);

            stage = "Marking setup complete (app.conf [install] is_configured=1)";
            await setIsConfigured(service, 1);

            stage = "Reloading the app so splunkd picks up the changes";
            await reloadApp(service);

            $(".success").show();
            stage = "Redirecting to the app home page";
            redirectToApp();
        } catch (e) {
            showErrors([describeError(stage, e)]);
        }
    }

    function validate(endpointUrl, apiToken) {
        const errors = [];
        if (!endpointUrl) {
            errors.push("Endpoint URL is required.");
        } else if (!/^https?:\/\/[^\s]+\..+/i.test(endpointUrl)) {
            errors.push("Endpoint URL must be a valid http:// or https:// URL with a host (e.g. https://api.example.com).");
        }
        if (!apiToken) {
            errors.push("API Token is required.");
        }
        return errors;
    }

    async function saveConfigSetting(service, properties) {
        const configCollection = service.configurations(APP_NAMESPACE);
        await configCollection.fetch();

        if (!fileExists(configCollection, SETTINGS_CONF_FILE)) {
            await createEntity(configCollection, SETTINGS_CONF_FILE);
            await configCollection.fetch();
        }

        const confFile = configCollection.item(SETTINGS_CONF_FILE);
        await confFile.fetch();

        if (!fileExists(confFile, SETTINGS_STANZA)) {
            await createEntity(confFile, SETTINGS_STANZA);
            await confFile.fetch();
        }

        const stanza = confFile.item(SETTINGS_STANZA);
        await stanza.fetch();
        await updateEntity(stanza, properties);
    }

    async function saveSecret(service, password) {
        const passwords = service.storagePasswords(APP_NAMESPACE);
        await passwords.fetch();

        const passwordKey = `${PASSWORD_REALM}:${PASSWORD_NAME}:`;
        const existing = passwords.item(passwordKey);

        if (!existing) {
            await createEntity(passwords, PASSWORD_NAME, {
                name: PASSWORD_NAME,
                password: password,
                realm: PASSWORD_REALM,
            });
        } else {
            await updateEntity(existing, { password: password });
        }
    }

    async function setIsConfigured(service, value) {
        const configCollection = service.configurations(APP_NAMESPACE);
        await configCollection.fetch();
        const appConf = configCollection.item("app");
        await appConf.fetch();
        const installStanza = appConf.item("install");
        await installStanza.fetch();
        await updateEntity(installStanza, { is_configured: value });
    }

    async function reloadApp(service) {
        const apps = service.apps();
        await apps.fetch();
        const app = apps.item(APP_NAME);
        await app.fetch();
        await app.reload();
    }

    function redirectToApp() {
        setTimeout(() => {
            window.location.href = `/app/${APP_NAME}`;
        }, 800);
    }

    // ---- small SDK helpers -------------------------------------------------

    function fileExists(collection, name) {
        return collection.list().some((entry) => entry.name === name);
    }

    function createEntity(collection, name, params) {
        return new Promise((resolve, reject) => {
            collection.create(params || name, (err, created) => {
                if (err) reject(err);
                else resolve(created);
            });
        });
    }

    function updateEntity(entity, properties) {
        return new Promise((resolve, reject) => {
            entity.update(properties, (err, updated) => {
                if (err) reject(err);
                else resolve(updated);
            });
        });
    }

    // ---- messaging -----------------------------------------------------

    function clearMessages() {
        $(".success").hide();
        $(".error").hide();
        $("#error_details").empty();
    }

    function showErrors(messages) {
        $(".error").show();
        const list = $("#error_details");
        messages.forEach((m) => list.append($("<li>").text(m)));
    }

    function describeError(stage, e) {
        let text = `Error during: ${stage}`;
        if (e && e.responseText) {
            try {
                const parsed = JSON.parse(e.responseText);
                const detail = (parsed.messages || [])
                    .map((m) => `${m.type}: ${m.text}`)
                    .join("; ");
                if (detail) text += ` — ${detail}`;
            } catch (_) {
                text += ` — ${e.responseText}`;
            }
        } else if (e && e.toString && e.toString() !== "[object Object]") {
            text += ` — ${e.toString()}`;
        }
        return text;
    }
});
