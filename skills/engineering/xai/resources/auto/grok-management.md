#### Grok Business / Enterprise

# License & User Management

**The Grok Business overview page at [console.x.ai](https://console.x.ai?utm_source=docs\&utm_medium=referral\&utm_campaign=grok-management\&utm_content=console-home) is your central hub for handling team licenses and user invitations.** As a team admin or user with appropriate permissions, you can buy licenses, invite new members, and manage access to ensure smooth collaboration.

Access this page by logging into [console.x.ai](https://console.x.ai?utm_source=docs\&utm_medium=referral\&utm_campaign=grok-management\&utm_content=console-home) and navigating to the overview section. Note that actions like purchasing or provisioning require specific permissions—see the [Permissions](#permissions-and-troubleshooting) section for details.

## Purchasing Licenses

Expand your team's capabilities by buying additional licenses directly from the overview page.

Available license types:

* **SuperGrok:** Standard business access with enhanced quotas and features.
* **SuperGrok Heavy:** Upgraded performance for demanding workloads.

To purchase:

1. On the overview page, select the license type and quantity.
2. Enter payment details if prompted .
3. Confirm the purchase—licenses will be added to your available pool for assignment.

Purchased licenses become immediately available for provisioning to users.

> [!NOTE]
>
>  Ensure your team's billing is set up correctly to avoid
> interruptions. Visit [Billing Settings](https://console.x.ai/team/default/billing?utm_source=docs\&utm_medium=referral\&utm_campaign=grok-management\&utm_content=billing) for more
> details.

## Inviting Users

Invite new team members to join your Grok Business workspace with a simple email invitation process.

To invite:

1. On the overview page, click "Invite users to Grok Business".
2. Enter the users' email addresses.
3. Select a license type to auto-provision upon acceptance .
4. Send the invitation—the user will receive an email with a link to activate their account.

Invited users gain access to the team workspace and basic team read permissions. (the latter is to allow for sharing conversations with your team members)

View invited users in the "Pending invitations" list on the overview page. As long as you have unassigned licenses available, they will be automatically provisioned when the user accepts.

## Assigning and Revoking Licenses

Once licenses are purchased or available, assign them to users for full team workspace access.

To assign:

1. From the overview page, select a user from your team list.
2. Choose an available license and assign it—access activates immediately.

To revoke:

1. Click the "..." for the user and choose "Unassign License" from the dropdown.
2. Confirm the action—the license returns to your available pool, and the user will no longer have access to your team's workspace.

Revocations take effect instantly, so ensure that you communicate changes to affected users.

> [!CAUTION]
>
>  Revoking a license removes team workspace access. Users will
> retain personal workspace functionality.

## Canceling Licenses

Reduce your team's commitment by canceling unused licenses.

To cancel:

1. On the overview page, select the license type and quantity to cancel.
2. Submit the cancellation request .

Cancellations may take a few days to process, and eligible refunds will be issued to your billing method. Canceled licenses are removed from your pool once processed.

## Sharing policy

Team admins decide how widely members can share Grok Business resources. A sharing policy sets the widest audience a member may pick for a given resource; members can always share more narrowly, never more broadly.

Configure these policies in [console.x.ai](https://console.x.ai?utm_source=docs\&utm_medium=referral\&utm_campaign=grok-management\&utm_content=console-home) under your team's **Sharing & Retention** settings, in the **Product Sharing** section. Each resource type carries its own policy: conversations, projects, and skills.

The levels form a ladder from most to least restrictive:

| Level | What members can do |
| --- | --- |
| Private | Sharing is turned off for that resource. |
| Team | Share with individual teammates and with the member's own team. Organization-wide and cross-team sharing are blocked. |
| Organization | Everything Team allows, plus sharing across teams and with everyone in the organization. |
| Public | Everything Organization allows, plus public links that anyone can open, including people outside the team. |

Public links apply to conversations only; projects and skills cap at Organization. By default, conversations and projects can be shared organization-wide, and skills start at Private.

Tightening a policy applies right away. Members can no longer create shares wider than the new ceiling, and access to existing shares beyond it is restricted to match.

## Permissions and Troubleshooting

Most management actions require specific role-based permissions:

* **Billing Read-Write:** Needed to purchase or cancel licenses.
* **Team Read-Write:** Needed to invite users or assign/revoke licenses.

These are typically granted only to team admins. If you lack permissions:

* Contact your team admin to request actions like license assignment or purchases.
* Admins can adjust permissions via the overview page's role settings.

If you encounter issues, such as invitations not provisioning due to insufficient licenses, purchase more or revoke unused ones first.

> [!NOTE]
>
>  For white-glove support, Enterprise upgrades, or permission issues, contact SpaceXSI sales at .

