# Nutanix Objects Manual: Nutanix Objects Browser Guide

## NUTANIX OBJECTS BROWSER

The Nutanix Objects Browser is a User Interface (UI) that helps users to directly launch the object store instance in a web browser and perform bucket and object-level operations. This eliminates the steps of logging into Prism Central and launching the Nutanix Objects service.The administrators use Prism Central to manage the object stores. When an administrator creates an object store, a default bucket (objectsbrowser) gets created. The static website hosting capability is already enabled on this default bucket. The Nutanix Objects Browser UI is hosted on this default bucket.To launch the Nutanix Objects Browser, select the object store and click

### Actions > Launch Objects

### Browser. Nutanix Objects Browser will be opened in a new window. You can also launch the Nutanix

Objects Browser from the buckets list page (See Viewing Buckets in Nutanix Objects on page 127).Alternatively, if you have multiple load balancer IP addresses for the object store, you can click any of the IP addresses to launch the Nutanix Objects Browser for that object store.

### Figure 48: Prism Central - Launch Nutanix Objects Browser

The administrator can also share the object store URL with the user to access the Nutanix Objects Browser UI. The URL can be formed using the Nutanix Objects Public IP address. http://objects-public-ipaddress/objectsbrowser Note: You can use http or https to access the Nutanix Objects Browser. Refer to the following image to understand the URL formation.

### Note:

- Refreshing the page logs out the user and cancels the pending and in-progress uploads.• Any uploads taking more than 60 minutes gets terminated and retried three times before

getting canceled.

### Browser Version Compatibility for Nutanix Objects Browser

Supported browser versions for Nutanix Objects Browser.

**Table 43: Browser Compatibility**

| Browser | Minimum Supported Version |
| --- | --- |

Google Chrome Microsoft Edge

| Safari | 15.4 Objects | Nutanix Objects Browser | |
| --- | --- |

| Browser | Minimum Supported Version |
| --- | --- |

Mozilla Firefox

### Administrator Workflow: Nutanix Objects Browser

The administrator needs to perform the following steps to grant access to the Nutanix Objects Browser to a particular user.

### About this task

### Note:

- Nutanix Objects Browser does not store the access and secret keys.• If the user refreshes the page with the refresh button of the web browser, they will need to

provide the credentials again, and any unsaved changes will be lost (including any pending or in-progress uploads). To grant access to Nutanix Objects Browser, follow these steps:

### Procedure

1. Add the IAM user. For more information, see Generating Access Key for API Users on page 94.After the administrator adds the user, the access and secret keys are generated. 2. (Optional) Grant access permissions (read only, full access, or custom) to the buckets. For more information, see Sharing a Bucket in Nutanix Objects on page 130.

### Note: This step is optional and can be performed if the administrator wants to share bucket of another

user with a new user. If the administrator does not perform this step, the buckets list will be empty for the IAM users. The users can still create buckets from the Nutanix Objects Browser UI. 3. Share the Nutanix Objects Browser URL with a user to launch the object store in a web browser. . Share the access and secret keys that you generated for the user. The user would require to enter the keys in the login page of the Nutanix Objects Browser UI.

### Figure 49: Nutanix Objects Browser Login Page

### What to do nextThe user can log on to the Nutanix Objects Browser UI and perform CRUD operations within the object

store. For example, create buckets, upload objects, assign a tag to objects, and so on. See Supported Operations in Nutanix Objects Browser on page 209.

### Launching the Nutanix Objects Browser

Access the Nutanix Objects Browser UI using the object store URL and the keys provided by the administrator.

### About this taskTo launch the Nutanix Objects Browser, follow these steps:

### Procedure

1. Open the object store URL shared by the administrator in a web browser. Objects | Nutanix Objects Browser |

### 2. In the Nutanix Objects Browser login page, enter the Access and Secret keys to access the object

store instance.

### Figure 50: Nutanix Objects Browser Login Page

Note: The Administrator shares the access and secret key. The object store is opened in the Nutanix Objects Browser.

### What to do nextYou can now perform CRUD operations on buckets and objects. See Supported Operations in Nutanix

Objects Browser on page 209. You can also change the mode from light to dark, logout, or see the version by clicking the user name at top-right corner.

### Supported Operations in Nutanix Objects Browser

Users can perform various operations on buckets and objects in the Nutanix Objects Browser UI.In Nutanix Objects Browser UI, users can create buckets, configure policies, enable versioning, and set quotas. Buckets serve as logical containers for objects, and users can manage access permissions, lifecycle rules, and replication settings. These operations help structure and secure object storage for scalable data management.

### Bucket Operations in Nutanix Objects Browser

After you log into the Nutanix Objects Browser, all the buckets that the administrator shared with you are listed with creation date and owner information.You can perform the following operations at the bucket level:

| • | Create Bucket: Allows you to create buckets. For more information, see Creating an S3 Bucket Using |
| --- | --- |

Nutanix Objects Browser on page 210. Objects | Nutanix Objects Browser |

| • | Lifecycle: Allows you to create lifecycle rules.Click the name of the bucket, and then click the Lifecycle option at the left pane. For information, see |
| --- | --- |

Creating Lifecycle Rules Using Nutanix Objects Browser on page 218.

| • You can use the | Actions list to update bucket properties, host a static website, configure CORS, and |
| --- | --- |

delete a bucket.

| • | Update: Allows you to update the bucket properties.To update bucket properties, select the bucket, and then click Actions > Update. |
| --- | --- |
| • | Delete: Allows you to delete a bucket.To delete a bucket, select the bucket, and then click Actions > Delete. |

### Note: You can only delete an empty bucket. In the case of a version-enabled bucket, the delete

operation performed on an object is not permanent. The object gets removed from the list and moved

| to Recycle Bin and a Delete Marker gets created. For more information, see | Understanding Object |
| --- | --- |

Versions on page 236.

| • | Share: Allows you to share the bucket.To update bucket properties, select the bucket, and then click Actions > Share. |
| --- | --- |
| You can share a bucket with multiple users and provide permissions such as | Read only, Full |

### Access, and Custom. For more information see, Sharing a Bucket Using Nutanix Objects Browser

on page 216.

| • | Static Website: Allows you to configure a bucket to host a static website.You can configure a bucket for website hosting, and then upload your website files (objects) to the |
| --- | --- |

bucket. For more information see, Configuring a Bucket for Static Website Hosting using Nutanix Objects Browser on page 221.

| • | CORS: Allows you to configure CORS on a bucket.This allows the bucket to service cross-origin requests. For more information see, Configuring CORS |
| --- | --- |

on a Bucket using Nutanix Objects Browser on page 222.

### Creating an S3 Bucket Using Nutanix Objects Browser

You can create, modify, and delete a bucket using the Nutanix Objects Browser.

### About this task

### Note:

- Make sure that the bucket names are unique for all users.• You cannot configure a WORM bucket while creating a bucket. You can edit WORM policies

only after creating a bucket.

- You cannot enable multi-protocol access on an S3 bucket.• NFS buckets cannot be created using Nutanix Objects Browser.• While creating a new versioned bucket, the system will automatically create a lifecycle policy to

expire the delete markers if they are the only remaining object versions. To create and configure a bucket, follow these steps: Objects | Nutanix Objects Browser |

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. Click the name of the object store in which you want to create a bucket and launch the Nutanix Objects Browser.For more information on launching the Nutanix Objects Browser, see Launching the Nutanix Objects Browser on page 208.The Object Store page appears. 4. Click Add Bucket > Create Bucket .

| The | Create Bucket window appears. |
| --- | --- |

### Figure 51: Create Bucket Window

5. On the General Settings section, type a name for your bucket. For more information on naming buckets, see Buckets Naming Conventions.

### 6. (Optional) On the Object Versions section, configure the following:

For more information on versions, see Object Versioning.

| a. | Enable Versioning: Select this check box to enable versioning of objects.This allows you to keep all versions of an object in the same bucket. For more information on |
| --- | --- |

versions, see Object Versioning.To apply life cycle policy with versioning enabled, see Rules for Expiration based Lifecycle Policy on page 107. Note: Enabling versioning allows you to recover objects from accidental deletion and overwrite. Objects | Nutanix Objects Browser |

### 7. (Optional) On the Lifecycle Policies section, select the following:

For more information, see Lifecycle Policies.

| a. | Expire current objects version after:: Select to type a time period after which the current version |
| --- | --- |

of the object expires.You can specify the number in days, months, or years.

| b. | Expire previous objects versions after: Select this check box to enter a time period to delete all |
| --- | --- |

the previous versions of the objects. This option is available only if versioning is enabled.You can specify the number in days, months, or years.

### Note:

- If versioning is not enabled, the current object is deleted permanently. When you enable

versioning, multiple versions of the same object are maintained in a bucket.

- NFS buckets cannot be created using Nutanix Objects Browser.

8. Click Save. The bucket is created successfully.

### What to do nextAfter creating a bucket, you can create objects through S3 APIs and manage them. For more information,

see Supported S3 APIs.You can also perform various actions on a bucket, such as configuring static websites and configuring CORS on a bucket.

- For more information on creating lifecycle rules, see Creating Lifecycle Rules Using Nutanix Objects

Browser on page 218.

- For more information on configuring static websites, see Configuring a Bucket for Static Website

Hosting using Nutanix Objects Browser on page 221.

- For more information on configuring CORS on a bucket, see Configuring CORS on a Bucket using

Nutanix Objects Browser on page 222.

### Adding External Bucket

You can add an existing external bucket to the federated namespace.

### Before you beginTo add an external bucket, following permissions are needed:

- "s3:ListBucket"• "s3:*Object".

### About this taskTo add an external bucket, follow these steps:

### Procedure

1. Log on to the Prism Central web console. . In the Application Switcher, click Objects.3. Click the name of the object store in which you want to add a bucket and launch the Nutanix Objects Browser.For more information on launching the Nutanix Objects Browser, see Launching the Nutanix Objects Browser on page 208.The Object Store page appears. 4. Click Add Bucket > Add External Bucket .

| The | Add External Bucket window appears. |
| --- | --- |

### Figure 52: Add External Bucket Window

5. On the General Settings section, type a name of existing external bucket.

### Note: Nutanix recommends that external buckets use a differentiating common prefix in order to

distinguish external buckets from other buckets on Nutanix Objects. 6. On the Endpoint, select an appropriate external object store.7. Provide Access Key and Secret Key. These credentials are specific to the bucket that is being added. 8. Click Add. External bucket is added successfully.

### What to do nextAfter adding an external bucket, you can share the bucket with same permissions with another user. For

more information, see Sharing a Bucket Using Nutanix Objects Browser on page 216.

- By default, full access permissions are granted locally.• Final permissions are governed by the bucket policies defined in S3.After sharing a bucket using Nutanix Objects Browser, you can upload Nutanix Objects to S3 bucket, for

more information see,You can also perform various actions on a bucket, such as configuring static websites and configuring CORS on a bucket.

- For more information on creating lifecycle rules, see Creating Lifecycle Rules Using Nutanix Objects

Browser on page 218.

### Uploading Nutanix Objects to External Buckets

You can upload objects to external buckets. Objects | Nutanix Objects Browser |

### About this taskTo upload objects to external buckets, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. Select the object store and launch the Nutanix Objects Browser.4. Click the Buckets tab, and in the Buckets table, select the bucket to upload Object.5. Click Upload Nutanix Objects > Select Files . Upload Nutanix Objects window appears. 6. Click Close. The objects are in the bucket.

### What to do next(optional) After uploading Nutanix Objects to the external bucket, you can remove the bucket and the

endpoint. For more information, see Deleting a Bucket in Nutanix Objects on page 139.

### Bucket Operations in Nutanix Objects Browser for External Bucket

After you log into the Nutanix Objects Browser, all the buckets that the administrator shared with you are listed with creation date and owner information.You can perform the following operations at the bucket level:

| • | Create Bucket: Allows you to create buckets. For more information, see Creating an S3 Bucket Using |
| --- | --- |

Nutanix Objects Browser on page 210.

| • | Lifecycle: Allows you to create lifecycle rules.Click the name of the bucket, and then click the Lifecycle option at the left pane. For information, see |
| --- | --- |

Creating Lifecycle Rules Using Nutanix Objects Browser on page 218.

| • You can use the | Actions list to update bucket properties, and delete a bucket. |
| --- | --- |
| • | Update: Allows you to update the bucket properties.To update bucket properties, select the bucket, and then click Actions > Update. |
| • | Delete: Allows you to delete a bucket.To delete a bucket, select the bucket, and then click Actions > Delete. |

Note: External buckets are only removed from the namespace, not deleted from the S3 endpoint.

| • | Share: Allows you to share the bucket.To update bucket properties, select the bucket, and then click Actions > Share. |
| --- | --- |
| You can share a bucket with multiple users and provide | Full Access permission. For more |

information see, Sharing a Bucket Using Nutanix Objects Browser on page 216.

### Updating a Bucket using Nutanix Objects Browser

After you create a bucket, you can update the bucket settings using Nutanix Objects Browser. Objects | Nutanix Objects Browser |

### About this task

### Note: Life cycle rules created for a bucket cannot be modified from here. To modify the lifecycle rules for a

| bucket, see | Creating Lifecycle Rules Using Nutanix Objects Browser on page 218. |
| --- | --- |

To update bucket settings, follow these steps:

### Procedure

1. Log on to the Nutanix Objects Browser. For more information on launching Nutanix Objects Browser, see Administrator Workflow: Objects Browser and Launching the Objects Browser. 2. From the object store, select the bucket that you need to update.3. Click tripple dot horizontal icon > Update.

### Figure 53: Update Bucket Window

### 4. On the Object Versions section, select one of the checkboxes to enable or disable versioning of

objects in the bucket.

| a. | Enable versioning: Select this check box to enable versioning on objects and to keep all the |
| --- | --- |

versions of the object on the same bucket. Note: Select this option to recover objects from accidental deletion or overwrite.

| b. | Suspend versioning: Select this check box to disable versioning of objects in a bucket.When you suspend versioning, accumulation of the new object versions is stopped. However, |
| --- | --- |

versions of objects already existing in the bucket are retained. 5. Click Done. The updated settings are applied to the bucket successfully. Objects | Nutanix Objects Browser |

### Sharing a Bucket Using Nutanix Objects Browser

You can modify user access to grant different permissions, including read-only, full, or custom. Before you beginThe ability to perform the share bucket operation is limited to the owners of the bucket. If a non-owner attempts to update the policy, they must have the necessary permissions granted by the owner or administrator in the bucket policy. Without these permissions, the non-owner receives an Access Denied error message.

### About this taskTo share a bucket, follow these steps:

### Procedure

1. Log on to the Nutanix Objects Browser. For information on launching the Nutanix Objects Browser, see Launching the Nutanix Objects Browser on page 208.All the buckets that the administrator shared with you are listed with the creation date and owner information in a new window. 2. Click the Buckets tab, and in the Buckets table, select the bucket to be shared.3. Click tripple dot horizontal icon > Share.

| The name of the bucket owner is visible under | Owner. You cannot edit the name of the bucket owner. |
| --- | --- |

Removing or changing the permission for the user who is a bucket owner does not affect the bucket. The owner is still allowed to do all the operations on a bucket. Objects | Nutanix Objects Browser |

### 4. Type the email address of the users or select Anyone with Link as a user and set the required

permission for that user.You can only add users with access keys. When you select

### Anyone with Link as a user, a public

bucket access link is provided. You can copy and share the link with any user to directly access the bucket from the web browser.

### Caution: Creating permissions with Anyone with Link as a user grants unauthenticated access,

exposing the bucket to anonymous users.

### Figure 54: Share Bucket Window

Nutanix supports the following list of permissions on a bucket:

| • | Read Only: Provides read-only access to the user. |
| --- | --- |
| • | Full Access: Provides all access to the user. |
| • | Custom: Provides customizing of varied levels of access to the user. Click Set under Permissions to |

set custom permission for a user. Modifying or revoking permissions for a user who is the bucket owner does not affect the bucket. The owner retains unrestricted access and can continue to perform all operations on the bucket.For more information on Bucket Permissions, see Bucket Access Policies.For more information on generating access keys, see Generating Access Key for API Users on page 94. 5. (Optional) To add more users and permissions, click + Add User and Permissions.

| • If you add | Anyone with Link as Users and grant them Full Access, the user is able to perform all |
| --- | --- |

operations without any authentication.

- To add a new user, you must enter the precise user name without any variations or errors.• You can enter multiple user names by providing space between each user's name.

6. (Optional) Click the delete icon to remove an existing set of users and permission.7. Click Save. The bucket is now shared with the listed users with the allotted permissions.

### Note: You cannot save a policy with an empty set of permissions. It is required to have at least one user

and permission. Objects | Nutanix Objects Browser |

### Creating Lifecycle Rules Using Nutanix Objects Browser

Lifecycle policy enables you to create or update a set of rules that define actions that Nutanix Objects applies to a group of objects within a bucket. With these policies, you can expire objects when no longer required. Before you beginSee Lifecycle Policies in Nutanix Objects on page 105.

### About this task

### Note: Rules or any updates to the rules get applied to the new objects that you create and do not apply to

the objects existing before the rule creation or update. To create a lifecycle rule, follow these steps:

### Procedure

1. Log on to the Nutanix Objects Browser.2. In the list of buckets, click the name of the bucket for which you want to create a rule.3. Click Lifecycle.4. Click Create Rule.

| When you have no rules created, you can directly import the XML. The | Import XML option is not |
| --- | --- |

available if you already have rules to avoid overwriting. Objects | Nutanix Objects Browser |

### 5. In the Scope section, do the following, and then click Next:

### Figure 55: Defining Scope for a Rule

| a. | Name: Enter a name that identifies the rule you are creating. |
| --- | --- |
| b. | Scope: Select the scope of the rule to either all objects of that bucket, or to tags and prefixes. |
| » | Prefix: You can enter only one prefix. |
| » | Tag: You can enter up to 10 tags in key value pair.Click |

+ Add Tag to add more tags. Objects | Nutanix Objects Browser |

### 6. In the Configure Rule section, do the following, and then click Next:

### Figure 56: Configuring Rule

You cannot configure the tiering endpoints from the Nutanix Objects Browser. You can configure it from the Nutanix Objects UI and the same is visible as Read Only in the Nutanix Objects Browser UI.

| a. | Expire Select which version to expire: Current version, Previous version, Multipart uploads, or |
| --- | --- |

Delete markers according to your requirement.

| • The | Previous version and Delete markers options appear only for a version-enabled bucket. |
| --- | --- |

- Duplicate expiration field is not allowed. For example, you cannot create two rules for the current

version expiration.

- Expiration of delete marker and the current version cannot be configured in a single rule.

### Tip: It is recommended to create a rule to automatically expire delete markers to enhance the

performance of your object stores. Multipart uploads expiration should not be specified with tag-based filters.

| b. | after last creation date Specify the number in days, months, or years after which that respective |
| --- | --- |

version of the object expires after last creation date of that object.You can click

### Add Action to add up to three expiration rules. You can create expiration rules for the

current version, the previous version, and multipart uploads.

| Click | Delete Icon to delete the rule. Objects | Nutanix Objects Browser | |
| --- | --- |

7. In the Review section, review the scope and actions, and then click Done.

| • You can select a bucket, and update, delete, disable, and enable the rule using the | Actions drop- |
| --- | --- |

down.

| • You can also export the multiple rules to an XML file by clicking | Export to XML. |
| --- | --- |

Actions listed are executed in sequence.The rule you just created gets enabled and appears in the Rules table.

### Configuring a Bucket for Static Website Hosting using Nutanix Objects Browser

You can use Nutanix Objects Browser to host a static website that has individual web pages with static content. To host a static website on Nutanix Objects, you can configure a bucket for website hosting, and then upload your website files (objects) to the bucket. When you configure a bucket as a static website, you enable static website hosting, and optionally, add an index document and an error page. You can upload files (such as index documents and error pages) to the bucket using the Nutanix Objects Browser or S3 browser. The S3 browser uses the S3 protocols by providing access and the secret key. You can also choose to redirect to a website. Once you have configured your bucket as a static website, you can access the bucket through the object store endpoints for your bucket.

### About this taskTo configure a bucket for static website hosting, follow these steps:

### Procedure

1. Log on to the Nutanix Objects Browser.2. In the Buckets table, select the bucket to configure it for static website hosting.3. Click tripple dot horizontal icon > Static Website.

| The | Configure Static Website window appears. |
| --- | --- |

4. By default, the endpoint is auto-populated when you click Save at the last step. For example, when an endpoint auto populates, the URL will be in the formatobjectstorename

| .domain/bucketname | . For example, objectstore.nutanix.com/teamobjects. |
| --- | --- |

However, if they have set up the DNS correctly, then you can also access the website withbucketname

| .objectstorename.domain | endpoint using HTTP or HTTPS. For example, https:// |
| --- | --- |
| teamobjects.objectstore.nutanix.com | . |

5. Click the Host Website or Redirect check box.

| » | Use this bucket to host a website: Select this option to use the bucket to host the website. |
| --- | --- |

Optionally, you can enter the name of the index document (for example, myindex.html) and an error page.An index document is a web page that Nutanix Objects returns when you request the root of a website. It is the default page that loads when you are not requesting any specific page. After you enable static website hosting for your bucket, you can upload an HTML file with the index document name (for example, myindex.html) to your bucket. For example, if you specify no object in the URL, then the website loads the index page (myindex.html) that you have configured. If you have not configured an index document, then a website access to the root will return an access denied error message.A custom error page is a web page that Object returns when an error occurs. For example, if you are trying to load an object that does not exist, the website loads the error page that you have configured. Objects | Nutanix Objects Browser |

| » | Redirect: Select this option to enter a website URL to redirect to that website. For example, when |
| --- | --- |

you try to access the bucket endpoint, you will be redirected to this website. The protocol used is either HTTP or HTTPS.

| To remove the static website configuration from the Nutanix Objects Browser, uncheck | Host website or |
| --- | --- |

redirect, and then click Save. 6. Click Save.

| An endpoint is auto-generated when you click | Save. This endpoint will be the object store endpoint for |
| --- | --- |

your bucket and is used as the website address.You can now use your bucket as a static website. You can use the endpoint to test your static website.

### Configuring CORS on a Bucket using Nutanix Objects Browser

Cross-Origin Resource Sharing (CORS) allows a web application loaded in one domain to access the restricted resources that are requested from another domain. Before you beginSee Cross-Origin Resource Sharing (CORS) in Nutanix Objects Overview on page 123. About this taskYou set this configuration on a bucket so that the bucket can service cross-origin requests.To configure CORS for a bucket, follow these steps:

### Procedure

1. Log on to the Nutanix Objects Browser.2. In the Buckets table, select the bucket to configure CORS.3. Click tripple dot horizontal icon > CORS.

| The | Configure CORS window appears. |
| --- | --- |

4. Type or copy and paste a configuration file, or edit an existing configuration. The configuration file must be an XML file. 5. Click Save. The CORS configurations are saved for the bucket.

### Adding Tags to a Bucket Using Nutanix Objects Browser

A tag is a label that you assign to a bucket, and it consists of a key and a value pair that you can define. You can add or remove tags from your buckets using the Nutanix Objects Browser.

### About this taskTo add tags to a bucket from the Nutanix Objects Browser, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects. . Click the name of the object store and launch the Nutanix Objects Browser. For information on launching the Nutanix Objects Browser, see Launching the Nutanix Objects Browser on page 208.The Object Store page appears. 4. In the Buckets table, select the bucket to add tags.5. Click tripple dot horizontal icon > Tags.

| The | Add Tags window appears. |
| --- | --- |

6. To add a new tag, click + add tag. The existing tags will be auto-populated. You can click the delete icon to delete a tag.

### Note:

- The maximum number of tags allowed per bucket is 50.• Tag keys can be up to 128 Unicode characters in length, and tag values can be up to 256

Unicode characters in length.

- Tag keys and values are case-sensitive.• Tag key should be unique and is a mandatory field, but tag value is optional.

A new row appears to fill the key value of the tag. 7. Enter the key and value.8. Click Save. The changes are saved.

### Bucket Summary in Nutanix Objects Browser

The Summary page allows you to view the list of various policies applied to the bucket.To view the

### Summary page of a bucket in an object store, click the name of the bucket in the buckets

| table, and then click | Summary. |
| --- | --- |

### Object Operations in Nutanix Objects Browser

You can perform the following operations on an object within a bucket in the Nutanix Objects Browser UI:

- Upload objects

For more information, see Uploading an Object Using Nutanix Objects Browser on page 227.

- Create new text object

For more information, see Creating a Text Object with the Nutanix Objects Visualizer in Nutanix Objects Browser on page 229.

- Create new folder

For more information, see Creating a Folder in Nutanix Objects Browser on page 230.

- Open an object: Open the objects and preview it.

Open objects in Nutanix Objects visualizer using a user-selected preview plug-in. The preview can be in the form of text, image, PDF, audio, or video.

- For more information, see Nutanix Objects Visualizer on page 224.

Objects | Nutanix Objects Browser |

- Download an object

For more information, see Nutanix Objects Visualizer on page 224.

- Copy sharing link with other users: Generate a link to the object that you can share with other users.

Other users can directly open the shared object using the link and perform actions depending on the permissions. Users are prompted to log on when they open the link.

- Add tags to an object

For more information, see Adding Tags to an Object Using Nutanix Objects Browser on page 231.

- View and manage the different versions of an object

For more information, see Understanding Object Versions on page 236.

- Query the CSV objects using the SQL query

For more information, see Querying CSV Nutanix Objects Using S3 Select in Nutanix Objects Browser on page 234.

- Delete an object

For more information, see Deleting an Object Using the Nutanix Objects Browser on page 235.

- Search an object

For more information, see Searching an Objects Using Nutanix Objects Browser on page 233. Object Naming ConventionsThe object key name is a sequence of UTF-8 characters not exceeding 1024 bytes.You can use the following characters to create an object key name or prefixes:

- Alphanumeric characters—lowercase letters (a-z), uppercase letters (A-Z), and numbers (0-9)• Special characters—forward slash (/), exclamation mark (!), hyphen (-), underscore (_), period (.),

asterisk (*), single quote ('), open parenthesis ((), and close parenthesis ()) Consider the following limitations before you create an object key name:

- Only the UTF-8 character set is supported.• The length of the object key name cannot exceed 1024 bytes.• Special characters, such as pound (#) and percent (%) are not supported.

### Note: Starting with the Nutanix Objects 5.1 release, Nutanix Objects no longer supports null bytes in object

names, aligning this change with Amazon Web Services Simple Storage Service (AWS S3) and Microsoft Azure.

### Nutanix Objects Visualizer

Within the Nutanix Objects Browser, you can directly interact with object content.You can perform the following tasks directly within the Nutanix Objects Browser:1. Preview Content: You can conveniently preview the content of the most common types of objects without the need for downloading.

### 2. Edit object content: For specific object types, you have the capability to edit content directly within the

objects visualizer.

### 3. Stream and view large objects: You now have the option to stream and view audio/ video without the

requirement for complete downloads. Objects | Nutanix Objects Browser |

### 4. Integrated Help/Support: When editing certain types of objects, you can access integrated help

and support directly within the Nutanix Objects Visualizer in the form of syntax highlighting and autocomplete snippets.

### Supported Object Types in Nutanix Objects VisualizerNutanix Objects visualizer supports following object types:

- Image• Text Code• Audio• Video• PDF

### ImageThis functionality enables the preview of common image formats, supporting the following: png, jpeg, jpg,

webp, gif, apng, avif, svg, bmp, ico.Supported operations:

- A user-friendly slider is available in the UI for zoom adjustments.• Rendering

- Images are rendered using the largest possible size based on the user's view port.• If image dimensions exceed the view port, they are scaled down, with the scaling reflected in the

zoom slider.

- The aspect ratio of the image is preserved during scaling to maintain image quality and prevent

distortion.

- Scaling is restricted to the image's actual resolution to prevent overstretching.

- Users can press and drag images on the screen to navigate and focus on specific areas of large

images.

- Download: For animated formats (like gif, avif), the plugin renders only the first frame, displaying a static

image in the preview. For more information, see Downloading Objects Using Nutanix Objects Browser on page 232.

- The plugin supports images up to 50 MB.

### Text CodeThis functionality facilitates the preview and editing of various text-based objects, supporting a broad range

of formats: log, xml, json, txt, csv, html, htm, js, ts, jsx, tsx, sh, php, scss, less, sass, css, py, c, cpp, java, swift, h, dart, gitignore, go, proto, json5, kotlin, ps1, scala, sql, yaml, yml, md, tf, rust, r, ini, man, me, xsl, coffee, scpt, xbap, ws, vbs, jsf, ejs, mjs, crt, dhtml, chtml, rjs, shtml, public, asp, aspx, xss, rhtml, do, csr, phtml, xhtml, xht, jss, cgi, pl, jsp, cs. Note: While editing large text objects, the performance might be degraded with an increase in object size. Supported operations:

- xml, json, html, js, jsx, ts, tsx, php, css, py, c, cpp, java, go, proto, sql, yaml, yml, md objects feature

color-coded syntax highlighting and collapsible sections for easy content navigation. Objects | Nutanix Objects Browser |

- You can initiate a search on content using CMD + F or CTRL + F for efficient find and replace

operations.

- Download: Downloads the object. For more information, see Downloading Objects Using Nutanix

Objects Browser on page 232.

- The plugin supports files up to 50 MB.

### AudioThis feature enables the preview of audio with seamless streaming support. It supports a variety of audio

formats, including mp3, ogg, and wav.Supported operations:

- Play: Initiate audio playback.• Pause: Halt audio playback.• Seek: Navigate to specific points in the audio.• Streaming: Audio content can be previewed without the need to download the entire object.• Download: Downloads the object. For more information, see Downloading Objects Using Nutanix

Objects Browser on page 232.

- There is no enforced size limit for preview since only partial content is loaded in the browser. Unused

content is periodically garbage collected to optimize resource usage. For more information on the limitations, see Limitations for Audio and Video Preview in Nutanix Objects Visualizer on page 227.

### VideoThis feature enables users to preview videos with seamless streaming support. It supports various video

formats, including mp4, webm, mov, and 3gp.Supported operations:

- Play: Initiate video playback.• Pause: Halt video playback.• Seek: Navigate to specific points in the video.• Expand the video to full-screen mode.• Picture-in-Picture Mode (in few browsers): Enjoy video playback in a compact overlay, allowing

multitasking.

- Download: Downloads the object. For more information, see Downloading Objects Using Nutanix

Objects Browser on page 232.

- Video content can be previewed without the need to download the entire object. Partial data is streamed

from object stores using content-range, rendering it directly in the browser. This feature enables users to preview even large videos without complete downloads.

- There is no enforced size limit for preview since only partial content is loaded in the browser. Unused

content is periodically garbage collected to optimize resource usage. For more information on the limitations, see Limitations for Audio and Video Preview in Nutanix Objects Visualizer on page 227. Objects | Nutanix Objects Browser |

### PDFThis functionality enables the preview of PDF objects and is compatible with browsers that support native

PDF rendering.Supported operations:

- Efficiently renders a limited number of pages by default, loading additional pages dynamically as the

user scrolls

- You can navigate directly to a desired page by specifying its page number.• You can adjust the PDF scale using either a slider or options available in the user interface.• You can click on and utilize links within PDFs.• This plug-in can support PDF up to 100 MB.• Compatibility for native and custom PDF viewers is ensured on the following browser versions:

- Google Chrome version 92 and above.• Microsoft Edge version 92 and above.• Safari version 15.4 and above.• Mozilla Firefox version 90 and above.

### Limitations for Audio and Video Preview in Nutanix Objects Visualizer

The following are the limitations for audio and video objects preview in Nutanix Objects Browser:

- Presigned URL Expiration: Nutanix Objects Visualizer relies on presigned URLs with a 15-minute

expiration time for loading object previews. This constraint limits the time frame during which the plugin can fetch and process audio/video content.

- Audio/Video Operations Timeframe: The plugin supports all operations on audio and video content only

within the 15-minute window of the presigned URL's validity. Beyond this timeframe, certain operations may be affected.

- Playback Disruption: Once the presigned URL expires, there is a potential for playback disruption.

Specifically, audio operations may encounter difficulties when attempting to resume playback from parts that were not downloaded or were subject to garbage collection.

- Limited Support for URL Renewal: Currently, the plugin does not support the renewal of expired

presigned URLs. This limitation impacts the ability to extend the timeframe for operations and playback beyond the initial 15-minute duration.

- Media Play Continuation: The plugin lacks built-in support for the seamless continuation of media

playback after the expiration of the presigned URL. This may result in interruptions, particularly when dealing with large audio files.

- Handling Extended Video/Audio Durations: Similar challenges may arise when attempting to play video

or audio files exceeding 15 minutes in duration for their full intended playback duration.

### Uploading an Object Using Nutanix Objects Browser

After creating a bucket, you can use the Nutanix Objects Browser to select and upload files or folders containing multiple files to a bucket.

### About this taskTo upload files or folders to a bucket, follow these steps:

Objects | Nutanix Objects Browser |

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. Click the name of the object store in which you want to create a bucket and launch the Nutanix Objects Browser.For information on launching the Nutanix Objects Browser, see Launching the Nutanix Objects Browser on page 208.The Object Store page appears. 4. Click the bucket name to which you want to upload files or folders.5. (Optional) To select and upload a file, click Upload Objects > Select Files

| The | Upload Objects window appears. |
| --- | --- |

### Note:

- This page displays the upload progress, the upload status, the bucket name, the object

name, the size of the object, and the actions that you can perform on the object. You can also search by object name, bucket name or prefix.

- Uploading objects is an asynchronous process. If the upload size is large, you can close

| the | Upload Objects window and perform other operations in the Object Browser UI. You |
| --- | --- |

can check the upload status by clicking the status icon in the top-right corner of the page.

- Multi-part upload is used to upload large files (more than 1 GB). This feature enables to

upload objects of any size to the object store. Depending upon the size of the file, upload may take a few minutes.

- If you refresh the page using the refresh button of the web browser, logout, or navigate to

any non-application URL, any unsaved changes will be lost. This includes any pending or in-progress uploads. A warning message appears if you try to log out or refresh the page Objects | Nutanix Objects Browser |

while an upload is in progress. Keep the Nutanix Objects Browser open until the uploads are complete.

### Figure 57: Upload Objects Window

| a. Click the | Cancel All Updates to cancel the updates that are in progress. |
| --- | --- |
| b. Click the | Close button or the X icon, to close the Upload Objects window. |

### 6. (Optional) To select and upload a folder and the files in the folder, click Upload Objects > Select

### FolderThe

Upload Objects window appears. 7. (Optional) Click Summary to view the bucket summary.

### What to do nextAfter you upload objects to the bucket, you can add a tag to an object, download an object, view and

manage the versions of an object, and also delete an object. For more information, see:

- Object Operations in Nutanix Objects Browser on page 223• Adding Tags to an Object Using Nutanix Objects Browser on page 231• Understanding Object Versions on page 236• Deleting an Object Using the Nutanix Objects Browser on page 235

### Creating a Text Object with the Nutanix Objects Visualizer in Nutanix Objects Browser

Create a text object in a bucket of an object store with the Nutanix Objects Visualizer in the Nutanix Objects Browser UI. Before you beginSee Nutanix Objects Visualizer on page 224.

### About this taskTo create a text object in the Nutanix Objects Browser, follow these steps:

### Procedure

1. Log on to the Prism Central web console. . From the Application Switcher Function, click Objects.3. Select the object store, click the Triple Dot Horizontal icon, and click Launch Objects Browser.4. Enter the access and the secret key. Note: The Administrator shares the access and secret key. For information on launching the Nutanix Objects Browser, see Launching the Nutanix Objects Browser on page 208.The Object Store page appears in the Nutanix Objects Browser. 5. Click the bucket name that contains the object.6. Click Create New > Text Object.

| A | New Text Object dialog box appears. |
| --- | --- |

7. Enter the object name and click Create. An object can be any form of text, including code or other written content. You can assign any file extension to the object name, and the system treats it as a text file.The text file opens up in Nutanix Objects Visualizer in edit mode. 8. Edit the text file and click Save. The system saves the text file and switches to view mode.

### 9. (Optional) Do any of the following:

| » | Edit: Click to edit the file. |
| --- | --- |
| » | Download: Click to download the file. |

10. (Optional) To go back to the Objects List page, click the Close icon.

| The newly created object appears in the | Objects List page. |
| --- | --- |

### What to do nextYou can perform other object-related tasks, such as creating a folder, uploading objects, managing

versions, and deleting an object. For more information, see the following references:

- Creating a Folder in Nutanix Objects Browser on page 230• Object Operations in Nutanix Objects Browser on page 223• Understanding Object Versions on page 236• Deleting an Object Using the Nutanix Objects Browser on page 235

### Creating a Folder in Nutanix Objects Browser

Create folders to organize and manage your data within a bucket of an object store in the Nutanix Objects Browser UI. About this taskYou can create multiple folders to organize the objects in a bucket.To create a folder in the Nutanix Objects Browser, follow these steps:

### Procedure

1. Log on to the Prism Central web console. . From the Application Switcher Function, click Objects.3. Select the object store, click the Triple Dot Horizontal icon, and click Launch Objects Browser.4. Enter the access and the secret key. Note: The Administrator shares the access and secret key with the user. For information on launching the Nutanix Objects Browser, see Launching the Nutanix Objects Browser on page 208.The Object Store page appears in the Nutanix Objects Browser. 5. Click the bucket name that contains the object. Note: You cannot create a new folder in NFS-enabled buckets. 6. Click Create New > Folder.

### Note:

- Folder names can only contain alphanumeric characters and specific special characters,

including exclamation marks, hyphens, spaces, underscores, dots, asterisks, single quotes, and parentheses.

- When you create a folder, Nutanix Objects Browser creates a 0-byte object named after the

specified folder, followed by a slash (/).

| A | New Folder dialog box appears. |
| --- | --- |

7. Enter the folder name and click Create.

| The new folder is created and appears in the | Objects List page. |
| --- | --- |

What to do nextYou can perform other object-related tasks, such as uploading objects, managing versions, and deleting an object. For more information, see the following references:

- Creating a Text Object with the Nutanix Objects Visualizer in Nutanix Objects Browser on page 229• Object Operations in Nutanix Objects Browser on page 223• Understanding Object Versions on page 236• Deleting an Object Using the Nutanix Objects Browser on page 235

### Adding Tags to an Object Using Nutanix Objects Browser

You can use the Nutanix Objects Browser to add tags to an object in a bucket.

### About this taskTo add tags to an object, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects. . Click the name of the object store and launch the Nutanix Objects Browser. For information on launching the Nutanix Objects Browser, see Launching the Nutanix Objects Browser on page 208.The Object Store page appears. 4. Click the bucket name that contains the object.5. To tag an object, select the object that you need to tag, and click Actions > Tags.

| The | Add Tags window appears. |
| --- | --- |

### Figure 58: Adding Tags

a. Enter the key and value pair.b. Click Add Tag to add more tags. Note: You can add up to 10 tags for an object.Click the delete icon against a key-value pair to delete it.

| c. Click | Save. |
| --- | --- |

What to do nextYou can perform other object-related tasks, such as uploading objects, managing versions, and deleting an object. For more information, see:

- Object Operations in Nutanix Objects Browser on page 223• Understanding Object Versions on page 236• Deleting an Object Using the Nutanix Objects Browser on page 235

### Downloading Objects Using Nutanix Objects Browser

You can download an object from the Nutanix Objects Browser.

### About this taskTo download an object, follow these steps:

Objects | Nutanix Objects Browser |

### Procedure

1. Log on to the Prism Central web console.2. From the Application Switcher Function, click Objects.3. Click the name of the object store and launch the Nutanix Objects Browser. For information on launching the Nutanix Objects Browser, see Launching the Nutanix Objects Browser on page 208.The Object Store page appears. 4. Click the bucket name that contains the object.5. Click Actions > Download. The object is downloaded. What to do nextYou can perform other object-related tasks, such as uploading objects, managing versions, and deleting an object. For more information, see the following references:

- Object Operations in Nutanix Objects Browser on page 223• Understanding Object Versions on page 236• Deleting an Object Using the Nutanix Objects Browser on page 235

### Searching an Objects Using Nutanix Objects Browser

Perform a prefix-based search for an object n the search bar of the Nutanix Objects Browser.

### About this taskYou can perform the search on the

### Objects List page and the Recycle Bin. The search keyword is case-

sensitive. You can enter any prefix as a search keyword and the objects starting with that keyword are listed. For example, if you search for the prefix copy, all objects whose names start with the keyword copy are listed.You can also search for exact name matches. For example, searching for copy.txt returns only that specific file. Files with similar names, such as copy.text, do not appear in the results.To search an object, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. From the Application Switcher Function, click Objects.3. Click the name of the object store and launch the Nutanix Objects Browser. For information on launching the Nutanix Objects Browser, see Launching the Nutanix Objects Browser on page 208.The Object Store page appears. 4. Click the bucket name that contains the object.

| The | Objects List page appears. Objects | Nutanix Objects Browser | |
| --- | --- |

5. In the search bar, type the prefix or the exact name as the search keyword.

### Note: The scope of search is limited to the current folder. For example, if you search from within the

BigData folder, the search results appear only from the objects in the BigData folder, and not from the entire list of objects. The system displays the search results. What to do nextYou can perform other object-related tasks, such as uploading objects, managing versions, and deleting an object. For more information, see the following references:

- Object Operations in Nutanix Objects Browser on page 223• Understanding Object Versions on page 236

### Querying CSV Nutanix Objects Using S3 Select in Nutanix Objects Browser

S3 select adds support for querying the CSV objects using SQL query and viewing the results. You can perform all S3 Select-compatible operations using the queries.

### Before you beginRead Nutanix Objects S3 Select API Overview on page 256 and Supported SQL Functions by Nutanix

Objects S3 Select on page 257 sections.

### About this taskTo perform S3 Select compatible operations using SQL query, follow these steps:

### Procedure

1. Launch the Nutanix Objects Browser. For information on launching the Nutanix Objects Browser, see Launching the Nutanix Objects Browser on page 208.The Object Store page appears. 2. Click the bucket name that contains the object.3. Select the CSV object to run the query on, and click Actions > Run SQL Query.

### Note:

- Querying of CSV objects with header information is only supported.• Query with column numbers is not supported.

| The | Run SQL Query on <CSV_object_ name> window appears. Objects | Nutanix Objects Browser | |
| --- | --- |

4. Enter the SQL query in the Query field and click Run Query.

### Note: You can only run the S3 Select supported queries in the Query field. See Supported SQL

Functions by Nutanix Objects S3 Select on page 257.

### Figure 59: Nutanix Objects Browser - Running SQL Query on the CSV Object

Note: The result is limited to the first 1000 rows.

| The result is displayed in a tabular format in the | Result section. |
| --- | --- |

5. (Optional) Click the Export link to export the result. What to do nextYou can perform other object-related tasks, such as uploading objects, managing versions, and deleting an object. For more information, see the following references:

- Object Operations in Nutanix Objects Browser on page 223• Understanding Object Versions on page 236

### Deleting an Object Using the Nutanix Objects Browser

You can use the Nutanix Objects Browser to delete objects in a bucket that you do not need.

### About this taskTo delete objects from a bucket, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. From the Application Switcher Function, click Objects.3. Click the name of the object store and launch the Nutanix Objects Browser. For information on launching the Nutanix Objects Browser, see Launching the Nutanix Objects Browser on page 208.The Object Store page appears. 4. Click the bucket name that contains the object.5. Select the objects that you need to delete, and click Actions > Delete. . Click Confirm, to confirm deletion of the objects.

### Note:

- For versioned buckets, the delete operation performed on an object is not permanent and

a delete marker is created. You can view the deleted objects, and if needed, retrieve them

| from the | Recycle Bin. |
| --- | --- |

- For non-versioned buckets, the delete operation performed on an object is permanent and

| the deleted object cannot be retrieved. Non-versioned buckets do not have a | Recycle Bin. |
| --- | --- |

- When you delete an object within a folder, the system lists the deleted object in the recycle

bin under the folder name. For example, if the file "object1.txt" is in the "BigData" folder, it appears in the recycle bin as "BigData/object1.txt." 7. Click Recycle Bin, select the objects to be deleted permanently and click Delete Permanently.

### Figure 60: Delete Nutanix Objects

### Caution:

| • The | Delete Permanently action permanently deletes all versions of the selected objects. |
| --- | --- |

You cannot recover the deleted objects after you perform this operation.

- Any new version added to an object that is currently being deleted, gets automatically

deleted. What to do nextYou can perform other object-related tasks, such as uploading objects and folders, managing versions, and adding tags to an object. For more information, see:

- Object Operations in Nutanix Objects Browser on page 223• Understanding Object Versions on page 236• Adding Tags to an Object Using Nutanix Objects Browser on page 231

### Understanding Object Versions

### Versions allow you to view and manage the different versions of an object. From the Versions page, you

can revert, delete, and download the different versions of an object. These options are available only for version-enabled buckets.For more information on versions, see Object Versioning.For more information on managing object versions, see Managing Object Versions Using Nutanix Objects Browser on page 237. Objects | Nutanix Objects Browser |

| When you delete a version-enabled object from the | Bucket page, it is removed from the objects list and |
| --- | --- |
| moved to the | Recycle Bin. |
| To view the latest version of an object, click the object name, and then click | View all versions. The latest |
| version is listed with a | Delete Marker banner in the Recycle Bin. |
| You can permanently delete any version by selecting the object version, and then by clicking the | Delete |
| button. You can also permanently delete the selected objects including all its versions from the | Recycle |

Bin. Select the objects, and then click Delete Permanently.

### Note:

- Selection of objects is limited to a page-by-page basis. If you select all objects listed in the

Recycle Bin, all the objects listed on the first page are deleted. For example, if the total number of objects in the Recycle Bin is 150, but the first page has 100 objects listed, then the 100 objects are deleted.

- For the selected objects, if any new version is added while the deletion is in progress, that

version is also deleted.

| To restore (revert) this object to any previous version, select the object, and then click | View all versions. |
| --- | --- |
| Select the version of the object, and then click | Revert. Now, the reverted version is the latest version |

visible in the objects list. You can also select the latest delete marker and delete it to restore the object to the last version.

### Figure 61: Managing Object Versions - Using Nutanix Objects Browser

### Note: Versions with a Delete Marker banner can only be deleted permanently. After deletion, they cannot be

reverted or downloaded.

### Managing Object Versions Using Nutanix Objects Browser

You can use the Nutanix Objects Browser to view and manage the versions of an object. Version allows you to revert an earlier version of an object to its latest version. You can also delete or download any version of the object.

### About this taskTo manage objects versions, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects. . Click the name of the object store and launch the Nutanix Objects Browser. For information on launching the Nutanix Objects Browser, see Launching the Nutanix Objects Browser on page 208.The Object Store page appears. 4. Click the bucket name that contains the object.5. Select the objects whose version you need to manage, and click Actions > Versions.

### Figure 62: Nutanix Objects Versions

### Note: The latest version of an object is indicated by appending the banner Latest appended to the

object name. 6. (Optional) Select a version that you need to revert as the latest version, and click Revert.

### Figure 63: Revert Version

The above image depicts an example, where employeeDetails.xlsx (1) is reverted to the latest version indicated by employeeDetails.xlsx (4) [Latest].

### Note:

- Using the revert feature, you can designate any earlier version of an object as its latest

version.

| • If you deleted an object from the | Buckets page, where all the objects within the bucket |
| --- | --- |
| are listed. On the | Versions page, you can select a version that does not have the Delete |
| Marker banner, and then click | Revert. The object gets restored on the Bucket page. |

7. (Optional) Select a version of the object that you need to delete, and click Delete. This action permanently deletes the selected version of the object. . (Optional) Select a version of the object that you need to download, and click Download. You cannot download the object version that is marked with a Delete Marker banner. The object is downloaded to the default download folder of your browser. 9. Click Close or the X icon, to close the window.

### User and Key Management in Nutanix Objects Browser

Nutanix Objects Browser also provides end users with access to the Object service directly.With the Nutanix Objects Browser, you can also enable IT consumers within your organization, such as development, test, or devops team to manage their own access and secret keys directly through the Nutanix Objects Browser, without requiring intervention from a Prism Central Administrator. Roles for User and Key Mangement in Nutanix Objects BrowserThe following roles are available when configuring User and Key management in Nutanix Objects Browser.1. Prism Central Administrator: Use the Prism Central administrator role to set up access control policies, delegating management of bucket access and secret keys to end users.

### 2. Sub Admin: Use the sub-admin role to manage access keys on behalf of a specific group of users. A

user with a sub admin role optionally creates new users if additional permissions are granted.

### 3. End user: Use the end user role for direct access to Object storage through the browser interface or an

external S3 client.

### Prerequisites for User and Key Management in Nutanix Objects Browser

Deployment prerequisites, port configurations, and IAM roles requirements for user and key management in Nutanix Objects Browser.Ensure that you meet the following requirements before configuring the user and key in the Nutanix Objects Browser:

- Required IAM roles with appropriate permissions are configured.• The client system that opens the Nutanix Objects Browser must be able to reach the Prism Central IP

over port 9440 for Active Directory/LDAP authentication.

- The Nutanix Objects Storage network must connect to Prism Central over port 8443 to perform IAM API

operations.

### Limitations for User and Key Management in Nutanix Objects Browser

The following limitations apply when you configure the user and key in Nutanix Objects Browser:

- Only Active Directory (AD) or LDAP users can log in to the Nutanix Objects for user and key

management. Local users are not supported.

- Only the IAM users imported from AD or LDAP can be managed from Nutanix Objects Browser. Local

IAM users cannot be managed from Objects Browser.

- Users can only view, manage, or delete the access keys that they create. Keys created using the Prism

Central UI are owned by the admin and are not visible or editable by other users—even if those keys are associated with their username.

- A user who creates keys for another user becomes the owner of those keys. As a result, the recipient

user cannot manage those keys in Nutanix Objects Browser.

- The permissions required for user and key management in Nutanix Objects Browser are different from

those required for managing IAM users and keys from the Prism Central Objects page. Objects | Nutanix Objects Browser |

### User and Key Management in Nutanix Objects Browser Work flow

Configuring user and key in Nutanix Objects Browser feature enables the end users to manage their own access key and secret keys.The Nutanix Objects Browser user and key feature is an extension of the existing access key management capabilities using identity and access management (IAM) in the Prism Central interface. The primary use cases are the following:1. Support for end users to manage their own access keys, this is applicable in both of the following cases

- The end user already exists in IAM.• The end user does not exist in IAM.

2. Support for delegated sub-admins to manage access keys for other users.

**Table 44: Nutanix Objects Browser Workflow for Configuring Permissions and Policies**

| Sequence | Tasks Description Configure roles for admins or |
| --- | --- |
| the end user with specific set of | • For configuring role, see |
| permissions. | Creating a Custom Role for End User on page 240 • For a Sub-admin managing the keys,Creating a Role for Sub-Admin on page 243 Configuring access control policy • For an existing user, see Creating an Authorization Policy for Configurable Access (for an existing user) on • For a non existing user, see Creating an Authorization Policy for Configurable Access (for a non existing user) on • For a Sub-admin managing the keys, Creating an Authorization Policy for a Sub- Admin on page 244 |
| Managing IAM Keys | For more information, see Managing IAM Keys using LDAP on page 245 |

### Creating a Custom Role for End User

You can create a custom role for an end user.

### About this taskTo create the custom role for an end user in the Prism Central UI, follow these steps:

Objects | Nutanix Objects Browser |

### Procedure

1. Log in to Prism Central as an administrator.2. From the Application Switcher Function, select the Admin Center application and from the

| navigation bar, select | IAM. |
| --- | --- |

3. Select the Roles tab.4. Click Create Role > New Role.5. In the Role Name field, enter a unique name for the role.6. (Optional) In the Description field, enter a description.7. In the Entity Type filter the individual operations available for a specific resource type, select the Operation from the drop down list and in the search box enter the name of the resource.

### Note: The following permissions are required:

- View User• View User Key• Create User Key• Delete User Key

### 8. Click the plus icon to add an individual operations to the role.9. Do one of the following:

| » To create the role, click | Save. |
| --- | --- |
| » To create the role and attach the authorization policy, click | Save & Create Authorization Policy. |

### What to do next

- After creating a custom role for existing user, you can create authorization policy. For more information,

see Creating an Authorization Policy for Configurable Access (for an existing user) on page 241 section.

- After creating a custom role for non-existing user, you can create authorization policy. For more

information, see Creating an Authorization Policy for Configurable Access (for a non existing user) on page 242 section.

### Creating an Authorization Policy for Configurable Access (for an existing user)

Create an authorization policy with full access to all entity types and instances for the added users in the associated role. Before you beginCreate a custom role for an end user to generate their own user key. For more information, see Creating a Custom Role for End User on page 240.

### About this taskTo create an authorization policy, follow these steps:

Objects | Nutanix Objects Browser |

### Procedure

1. Log in to Prism Central as an administrator.2. From the Application Switcher Function, select the Admin Center application and from the

| navigation bar, select | IAM. |
| --- | --- |

3. Select the Authorization Policies tab.4. Click + Create Authorization Policy.5. (Optional) To edit the name, click the pencil icon next to the default name, edit the name, and click the checkmark icon.

### 6. On the Select Role search box, enter the built-in or custom role's name and select the role from the

list of suggestions. The system displays role details for the selected role.

### 7. Click Next.8. Select Configure access: select entity type & instances and follow these steps:

| a. From the | Entity Type dropdown list, select the entity type. |
| --- | --- |

The list of available entities depends on the role that you select.

| b. From the | Filters dropdown list, select the filter type. |
| --- | --- |
| c. On the | Search box, select the specific entities that belongs to the selected entity type. |

### 9. To allow the assigned users to access any new entity instances created by them, select the Allow

user access to entities created by them checkbox. 10. Click Next.11. From the Users dropdown list, select the active directory the user belongs to.12. On the Search box, enter the first few letters of the user or user group's name and select the correct user or user group from the list of suggestions. Note: The scope and identities must have the same user. 13. Click Save.

### What to do nextAfter creating an authorization policy, you can manage IAM keys. For more information, see Managing IAM

Keys using LDAP on page 245

### Creating an Authorization Policy for Configurable Access (for a non existing user)

Create an authorization policy with full access to all entity types and instances for the added users in the associated role. Before you beginCreate a custom role for an end user to generate their own user key. For more information, see Creating a Custom Role for End User on page 240.

### About this taskTo create an authorization policy, follow these steps:

Objects | Nutanix Objects Browser |

### Procedure

1. Log in to Prism Central as an administrator.2. From the Application Switcher Function, select the Admin Center application and from the

| navigation bar, select | IAM. |
| --- | --- |

3. Select the Authorization Policies tab.4. Click + Create Authorization Policy.5. (Optional) To edit the name, click the pencil icon next to the default name, edit the name, and click the checkmark icon.

### 6. On the Select Role search box, enter the built-in or custom role's name and select the role from the

list of suggestions. The system displays role details for the selected role. 7. Click Next.8. Leave Define Scope empty and click Next.9. From the Users dropdown list, select the active directory the user belongs to.10. On the Search box, enter the first few letters of the user or user group's name and select the correct user or user group from the list of suggestions.

### 11. Click Save

End user is created in IAM.

### 12. Edit the newly created policy, follow these steps:

| a. From the | Entity Type dropdown list, select the entity type. |
| --- | --- |

The list of available entities depends on the role that you select.

| b. From the | Filters dropdown list, select the filter type. |
| --- | --- |
| c. On the | Search box, select the specific entities that belongs to the selected entity type. |
| d. To allow the assigned users to access any new entity instances created by them, select the | Allow |

user access to entities created by them checkbox. 13. Click Save.

### What to do nextAfter creating an authorization policy, you can manage IAM keys. For more information, see Managing IAM

Keys using LDAP on page 245

### Creating a Role for Sub-Admin

You can create a custom role for a sub-admin managing keys for a set of users.

### About this taskTo create a custom role for sub-admin in Prism Central, follow these steps:

### Procedure

1. Log in to Prism Central as an administrator. Objects | Nutanix Objects Browser |

### 2. From the Application Switcher Function, select the Admin Center application and from the

| navigation bar, select | IAM. |
| --- | --- |

3. Select the Roles tab.4. Click Create Role > New Role.5. In the Role Name field, enter a unique name for the role.6. (Optional) In the Description field, enter a description7. In the Entity Type filter, the individual operations available for a specific resource type, select the Operation from the drop-down list, and in the search box, enter the name of the resource.

### Note: The following permissions are required:

- View User• View User Key• Create User Key• Delete User KeyThe following permissions are optional and not required for managing existing users but these

permissions are required for creating new users:

- Create User• View Directory Service• Search Directory Service

### 8. Click the plus icon to add an individual operations to the role.9. Do one of the following:

| » To create the role, click | Save. |
| --- | --- |
| » To create the role and attach the authorization policy, click | Save & Create Authorization Policy. |

What to do nextAfter providing the required permissions in the IAM role, you can create an authorization policy. For more information, see Creating an Authorization Policy for a Sub-Admin on page 244.

### Creating an Authorization Policy for a Sub-Admin

Create an authorization policy with full access to all entity types and instances for the added users in the associated role. Before you beginCreate a custom role for sub-admin managing keys for a set of users. For more information, see Creating a Role for Sub-Admin on page 243.

### About this taskTo create an authorization policy, follow these steps:

Objects | Nutanix Objects Browser |

### Procedure

1. Log in to Prism Central as an administrator.2. From the Application Switcher Function, select the Admin Center application and from the

| navigation bar, select | IAM. |
| --- | --- |

3. Select the Authorization Policies tab.4. Click + Create Authorization Policy.5. (Optional) To edit the name, click the pencil icon next to the default name, edit the name, and click the checkmark icon.

### 6. On the Select Role search box, enter the built-in or custom role's name and select the role from the

list of suggestions. The system displays role details for the selected role.

### 7. Click Next.8. Select Configure access: select entity type & instances and follow these steps:

| a. From the | Entity Type drop-down list, select the entity type. |
| --- | --- |

Select the list of users that a sub-admin can manage.

| b. From the | Filters drop-down list, select the filter type. |
| --- | --- |
| c. On the | Search box, select the specific entities that belongs to the selected entity type. |

### 9. To allow the assigned users to access any new entity instances created by them, select the Allow

user access to entities created by them checkbox. 10. Click Next.11. From the Users drop-down list, select the sub-admin user.

| The user group selected will be able to manage the entities selected in the | Define Scope (step 8). |
| --- | --- |

12. Click Save.

### What to do nextAfter creating an authorization policy, you can manage IAM keys. For more information, see Managing IAM

Keys using LDAP on page 245

### Managing IAM Keys using LDAP

You can use the object store URL to launch the Nutanix Objects Browser UI and use the Active Directory or LDAP credentials to manage IAM keys. Before you beginCreate a custom role for an end user to generate their own user key. For more information, see Creating a Custom Role for End User on page 240.Create a custom role for sub-admin to manage keys for a set of users. For more information, see Creating a Role for Sub-Admin on page 243.

### About this taskTo launch the Nutanix Objects Browser, follow these steps:

Objects | Nutanix Objects Browser |

### Procedure

1. Open the Object Browser URL shared by the administrator in a web browser.2. In the Nutanix Objects Browser login page, click Login with LDAP/AD

### Note: If you open the Objects Browser URL in the same browser where you're already logged in to

Prism Central with an AD/LDAP account, you might be automatically signed in to Objects Browser with the same credentials. To avoid this, open Objects Browser in a different browser or use an incognito/ private browsing window. The Prism Central Login page appears. 3. Log on to Prism Central using the created policy credentials.

| The | Access Keys table appears with the current policy. |
| --- | --- |

4. Click Add Key, generate the access key for the user.

| a. (Optional) Click | Add Key at the top to add new users. |
| --- | --- |
| b. (Optional) Click | Add Key next to the desired user to add a key for that specific user. |

### 5. You can do one of the following actions :

| » | Add Key: Click this button to generate the access key for the user. |
| --- | --- |
| » | Delete: Select the access key and click this button to delete the access key. |

Note: A logged-in user can manage only the access keys created by that user. Objects | Nutanix Objects Browser |
