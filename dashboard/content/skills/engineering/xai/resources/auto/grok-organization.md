+++
title = "grok-organization"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

#### Grok Business / Enterprise

# Organization Management

**Organizations provide a higher-level governance structure for enterprise customers, encompassing multiple console teams under unified IT controls.** Available only to Enterprise tier subscribers, organizations enable centralized management of users, teams, and security features like SSO.

Access the organization dashboard by visiting [console.x.ai/organization](https://console.x.ai/organization?utm_source=docs\&utm_medium=referral\&utm_campaign=grok-organization\&utm_content=organization). This page is restricted to organization admins.

> [!NOTE]
>
>  Organizations are exclusive to the Enterprise tier. Contact SpaceXSI
> sales to upgrade if needed.

## Understanding Organizations

An organization acts as an overarching entity that groups related console teams, ideal for large enterprises with multiple business units or departments.

Key features:

* **Domain Association:** Link your organization to a specific email domain (e.g., @yourcompany.com). Any user signing up or logging in with an email from this domain is automatically associated with the organization.
* **User Visibility:** Organization admins can view a comprehensive list of all associated users across teams on the `/organization` page.
* **Team Association:** Teams created by organization members are automatically linked to the organization and displayed in the dashboard for oversight.

This structure supports a multi-team architecture, allowing independent Grok Business or API teams while maintaining centralized governance, such as uniform access controls and auditing.

## Viewing Users and Teams

To view users:

1. Navigate to [console.x.ai/organization](https://console.x.ai/organization?utm_source=docs\&utm_medium=referral\&utm_campaign=grok-organization\&utm_content=organization).
2. Scroll to the "Users" section for a list of all domain-associated users, including their team affiliations and access status.

To view teams:

1. In the same dashboard, access the "Teams" section.
2. Review associated console teams, their members, and high-level usage metrics.

Use these views to ensure compliance, spot inactive accounts, or identify growth needs.

## Setting Up SSO

Secure and streamline logins by integrating Single Sign-On (SSO) with your preferred Identity Provider (IdP).

To configure SSO:

1. On the `/organization` page, click "Configure SSO".
2. Choose your IdP from the supported list (e.g., Okta, Azure AD, Google Workspace).
3. Follow the self-guided, IdP-specific instructions provided—each includes step-by-step setup, metadata exchange, and attribute mapping details.
4. Save your configuration and test SSO to confirm the functionality.

SSO setup is straightforward and tailored to common providers, ensuring quick deployment.

## Activating SSO and User Impact

Once configured, SSO will be activated and enforced organization-wide.

Post-activation:

* Users must log in via SSO on their next access.
* If a user selects "Log in with email" and enters a domain-associated address, (e.g., @yourcompany.com) the system automatically detects it and redirects to your IdP for authentication.
* Non-domain emails (e.g., @differentcompany.com) fall back to standard login methods.

This ensures seamless, secure access without disrupting workflows.

> [!NOTE]
>
>  Notify your users in advance about the SSO rollout to minimize
> support queries.

## Setting up SCIM

Automate user provisioning and deprovisioning by integrating System for Cross-domain Identity Management (SCIM) with your Identity Provider (IdP). Follow these steps to set up SCIM effectively.

### Step 1: Configure directory sync in your IdP

1. On the `/organization` page, click "Setup SCIM".
2. Follow the IdP-specific steps provided to connect your directory.
3. Create groups in your IdP that correspond to how you want to organize access in SpaceXSI—for example, `xai-engineering`, `xai-data-science`, or whatever fits your organizational structure.

This step ensures your directory is synced and your groups are ready for mapping.

### Step 2: Create roles

Define the roles your organization needs directly in the xAI console.

* Click **"Create Role"** to add a new role. Each role gets a name, a slug (used as a unique identifier), and an optional description.
* Create as many roles as you need to match your organizational structure—there is no limit.
* **Drag to reorder** roles by priority. Higher-priority roles take precedence when a user belongs to multiple groups.
* The **Member** role is always present at the bottom of the list as the default role assigned to users without any specified group.

You can also create and reorder roles later from the Provisioning tab on the organization page.

### Step 3: Map groups to roles

Map your IdP groups to the roles you created in the previous step.

1. On this step, click **"Assign Groups"** to open the management portal at **sso.x.ai**.
2. Click **"Configure role assignment"** to set up group-to-role mappings.
3. For each IdP group, select the corresponding SpaceXSI role.

This mapping aligns your IdP groups with SpaceXSI's role-based access controls so that users are automatically assigned the correct role when provisioned.

### Step 4: Configure roles with teams, permissions, and licenses

Assign your roles to the appropriate resources.

* **Teams:** Map each role to one or more console teams.
* **Permissions:** Assign the access control lists (ACLs) each role should have.
* **Licenses:** Associate the appropriate product license (e.g., Grok Business) with the role.

This step customizes access and entitlements based on your organizational needs.

### Step 5: Preview and activate SCIM

Before finalizing, review the changes.

* We provide a preview of what your organization will look like after activation.
* Confirm that members are assigned to the correct roles, those roles have the appropriate level of authorization, and the right licenses are applied.
* Once you feel confident everything is correct, click **"Activate"** to make SCIM your default provisioning system.

This verification ensures a smooth transition.

> [!WARNING]
>
>  SCIM is very disruptive. Users might lose or gain access to resources
> they did not have before. Notify your organization that you are undergoing this transition and
> **verify everything is correct during the preview stage before proceeding.**

### Managing roles after activation

After SCIM is activated, you can continue to manage roles from the **Provisioning** tab on the organization page:

* **Create new roles** using the "Create Role" button in the SCIM Roles section header.
* **Reorder role priority** using the "Reorder Priority" button.
* **Configure roles** by clicking the overflow menu on any role to update its teams, permissions, and licenses.

## Need Help?

For assistance with organization setup, SSO troubleshooting, or Enterprise features, contact SpaceXSI sales at [x.ai/grok/business/enquire](https://x.ai/grok/business/enquire).


