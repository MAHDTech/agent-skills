+++
title = "apis-and-integrations"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "nutanix-objects"
+++

# Nutanix Objects Manual: S3 APIs, Prometheus, and Integrations

## NUTANIX OBJECTS PROMETHEUS

## EXPORTER

The Nutanix Objects Prometheus Exporter provides a solution for Nutanix Objects observability. The Nutanix Objects Prometheus Exporter service runs on Prism Central. Using an external Prometheus server, stats can be periodically retrieved from the exporter service and visualized through Grafana dashboards. You can also use any other observability tools that can import stats in Prometheus format and store and visualize them.The exporter can be used to monitor the following stats:

- Object stores and buckets stats:

- Capacity consumed or usage• Number of objects• Number of buckets

- Performance stats (individual bucket level as well as cumulative object store level): Total requests/sec,

GET/sec, PUT/sec, throughput.

- Object store resource consumption stats: CPU and memory consumption for object store services -

Object Controller, Metadata service, Atlas, and Load Balancer.

### List of stats at bucket level

- nutanix_objectstore_total_object_data_gets• nutanix_objectstore_object_count• nutanix_objectstore_total_object_data_puts• nutanix_objectstore_storage_usage_bytes• nutanix_objectstore_total_deletes• nutanix_objectstore_total_batch_deletes• nutanix_objectstore_total_batch_delete_objects• nutanix_objectstore_total_batch_delete_with_versions• nutanix_objectstore_total_delete_with_versions• nutanix_objectstore_total_heads• nutanix_objectstore_total_lists• nutanix_objectstore_total_list_versions• nutanix_objectstore_total_multipart_lists• nutanix_objectstore_total_multipart_uploads• nutanix_objectstore_total_posts• nutanix_objectstore_total_requests_sec• nutanix_objectstore_total_select_content_ops• nutanix_objectstore_total_throughput_in_sec

Objects | Nutanix Objects Prometheus Exporter |

- nutanix_objectstore_total_throughput_out_sec

### List of stats at object store level

- nutanix_objectstore_total_object_data_gets• nutanix_objectstore_total_batch_deletes• nutanix_objectstore_total_batch_delete_objects• nutanix_objectstore_total_batch_delete_with_versions• nutanix_objectstore_total_delete_with_versions• nutanix_objectstore_num_buckets• nutanix_objectstore_num_objects• nutanix_objectstore_total_object_data_puts• nutanix_objectstore_total_deletes• nutanix_objectstore_total_heads• nutanix_objectstore_total_lists• nutanix_objectstore_total_list_versions• nutanix_objectstore_total_multipart_lists• nutanix_objectstore_total_multipart_uploads• nutanix_objectstore_total_posts• nutanix_objectstore_total_requests_sec• nutanix_objectstore_total_select_content_ops• nutanix_objectstore_total_throughput_in_sec• nutanix_objectstore_total_throughput_out_sec• nutanix_objectstore_usage_bytes

List of resource stats for services of an object storeCPU and memory usage for Atlas, Object Controller and Metadata Server.For more information on the integration steps with external Prometheus and Grafana, contact your Nutanix account team for further guidance. Objects | Nutanix Objects Prometheus Exporter |

## NUTANIX OBJECTS CRUD OPERATIONS

## BY USING S3 APIS

Create and manage buckets and objects by using S3 APIs in Nutanix Objects.Nutanix Objects provides S3-compatible object storage, enabling users to create, manage, and interact with buckets and objects using standard Amazon S3 APIs. You can interact with the service via REST API or AWS SDKs. All API interactions with Nutanix Objects are authenticated to ensure secure access. The object store supports Amazon S3 features such as tagging (key-value labels), S3 Select for partial content retrieval, and server-side encryption with customer-provided keys (SSE-C).

### Nutanix Objects Authentication

You can send requests to Nutanix Objects by using the REST API or the Amazon Web Services Software Development Kit (AWS SDK) wrapper libraries that wrap the underlying S3 REST API. Every interaction with Nutanix Objects is authenticated. In this authentication process, the identity of the requester who is trying to access Nutanix Objects is verified with a signature value. The signature value is generated from the AWS access keys (access key ID and secret access key) of the requester. This AWS access keys and endpoint URL is provided by the administrator to the user.If you are using the AWS SDK, the libraries compute the signature from the keys you provide. However, if you make direct REST API calls, the signature is computed from the request.For creating the buckets and objects, you need the following information from the administrator:

- Endpoint URL (Static IP address and the Port number)• Access key ID• Secret access keyOnce you get this information, you can import the SDK libraries, create a session and a client, and then

you can start making the requests (for example, creating buckets and objects).

### Note: Nutanix Objects supports the following:

- Signature Version 2 and Signature Version 4 (regular and pre-signed)• Streaming signed payloads for PUT requests with Signature Version 4

### Nutanix Objects Supported and Unsupported APIs

This section describes Nutanix Objects support for Amazon S3 API. The object store service is available on the following ports:

| • | HTTP: 80 |
| --- | --- |
| • | HTTPS: 443 |

### Note:

- Transport Layer Security (TLS) 1.2 is supported on Nutanix Objects.• Support for all CBC based ciphers are deprecated.

If you are using RSA certificate, the following ciphers are allowed. TLS/1.2 Objects | Nutanix Objects CRUD Operations by Using S3 APIs |

IANA : TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384 (OpenSSL : ECDHE-RSA-AES256-GCM- SHA384) IANA : TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256 (OpenSSL : ECDHE-RSA-AES128-GCM- SHA256) If you are using EC certificate, the following ciphers are allowed. TLS/1.2 IANA : TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384 (OpenSSL : ECDHE-ECDSA-AES256- GCM-SHA384) IANA : TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256 (OpenSSL : ECDHE-ECDSA-AES128- GCM-SHA256)

| • The default region for Nutanix Objects is | us-east-1. |
| --- | --- |

### Nutanix Objects Supported APIs

The following table lists the supported S3 API methods.

**Table 45: Supported S3 APIs and Parameters**

| Supported S3 APIs | Request Parameters/Request Headers/Request Elements/Request Body |
| --- | --- |
| PUT Bucket | Nutanix Objects Common Headers on page 253 + CreateBucketConfiguration, LocationConstraint (By default, LocationConstraint “us-east-1” is only supported.) |
| PUT Bucket Lifecycle | Nutanix Objects Common Headers on page 253 + Content-MD5 + AbortIncompleteMultipartUpload, And, Date, Days, DaysAfterInitiation, Expiration, Filter, ID, Key, LifecycleConfiguration, NoncurrentDays, NoncurrentVersionExpiration, NoncurrentVersionTransition, Prefix, Rule, Status |
| PUT Bucket Policy | Nutanix Objects Common Headers on page 253 + JSON string containing the policy contents containing the policy statements |
| PUT Bucket versioning | Status, VersioningConfiguration |
| Complete Multipart Upload | Nutanix Objects Common Headers on page 253 + CompleteMultipartUpload, Part, PartNumber, ETag, If-Match, If-None-Match Note: Nutanix Objects validates the If-Match and If-None-Match condition only at the start and end of an object upload. If it matches at both points, the request is accepted. Even if multiple competing requests modify the object state during the upload and cancel effects of each other, the conditional request is still considered valid and accepted. Objects | Nutanix Objects CRUD Operations by Using S3 APIs | |

| Supported S3 APIs | Request Parameters/Request Headers/Request Elements/Request Body |
| --- | --- |
| PUT Object - Copy | Nutanix Objects Common Headers on page 253 + x-amz-copy-source, x-amz-metadata-directive, x-amz-copy-source-if-match, x-amz-copy-source- if-none-match, x-amz-copy-source-if-unmodified- since, x-amz-copy-source-if-modified-since, x-amz- tagging, x-amz-tagging-directive, x-amz-meta-, x-amz-object-lock-legal-hold, x-amz-object-lock- mode, x-amz-object-lock-retain-until-date |
| DELETE Nutanix Objects | Single query string parameter delete + Common Headers + Content-MD5, Content-Length, Delete, Quiet, Object, Key, VersionId |
| GET Object | Nutanix Objects Common Headers on page 253 + Range, If-Modified-Since, If-Unmodified-Since, If- Match, If-None-Match, x-amz-object-lock-legal-hold, x-amz-object-lock-mode, x-amz-object-lock-retain- until-date |
| HEAD Object | Nutanix Objects Common Headers on page 253 + Range, If-Modified-Since, If-Unmodified-Since, If- Match, If-None-Match, x-amz-object-lock-legal-hold, x-amz-object-lock-mode, x-amz-object-lock-retain- until-date |
| List Multipart Uploads | Nutanix Objects Common Headers on page 253 + delimiter,max-uploads, key-marker, prefix, upload- id-marker |
| List Object Versions | Nutanix Objects Common Headers on page 253 + delimiter, key-marker, max-keys, prefix, version-id- marker |
| GET Bucket (List Nutanix Objects) Version 1 | Nutanix Objects Common Headers on page 253 + delimiter, marker, max-keys, prefix |
| GET Bucket (List Nutanix Objects) Version 2 | Nutanix Objects Common Headers on page 253 + delimiter, max-keys, prefix, list-type, continuation- token, fetch-owner, start-after |
| List Parts | Nutanix Objects Common Headers on page 253 + uploadId, max-parts, part-number-marker |
| PUT Object | Nutanix Objects Common Headers on page 253 + Content-Length, Content-MD5, If-Match, If-None- Match, Expect, x-amz-tagging, x-amz-meta-, x-amz- object-lock-mode, x-amz-object-lock-retain-until- date, x-amz-object-lock-legal-hold Note: Nutanix Objects validates the If-Match and If-None-Match condition only at the start and end of an object upload. If it matches at both points, the request is accepted. Even if multiple competing requests modify the object state during the upload and cancel effects of each other, the conditional request is still considered valid and accepted. Objects | Nutanix Objects CRUD Operations by Using S3 APIs | |

| Supported S3 APIs | Request Parameters/Request Headers/Request Elements/Request Body |
| --- | --- |
| INITIATE Multipart Upload | Nutanix Objects Common Headers on page 253 + x-amz-meta-, x-amz-tagging, x-amz-object-lock- legal-hold, x-amz-object-lock-mode, x-amz-object- lock-retain-until-date |
| UPLOAD Part | Content-Length, Content-MD5, Expect |
| Upload Part - Copy | x-amz-copy-source, x-amz-copy-source-range, x- amz-copy-source-if-match, x-amz-copy-source- if-none-match, x-amz-copy-source-if-unmodified- since, x-amz-copy-source-if-modified-since |
| PUT Bucket Object Lock Configuration | Nutanix Objects Common Headers on page 253 + ObjectLockEnabled Rule |
| DELETE Bucket | Nutanix Objects Common Headers on page 253 + Bucket |
| Delete Multiple Nutanix Objects | Nutanix Objects Common Headers on page 253 + Bucket, Delete, Quiet, Nutanix Objects, Key, VersionId |
| GET Object Retention | Nutanix Objects Common Headers on page 253 + Bucket, Key, VersionId |
| PUT Object Retention | Nutanix Objects Common Headers on page 253 + Bucket, Key, VersionId, Retention, Mode, RetainUntilDate |
| PUT Object Legal Hold | Nutanix Objects Common Headers on page 253 + Bucket, Key, versionId, LegalHold, Status |
| GET Object Legal Hold | Nutanix Objects Common Headers on page 253 + Bucket, Key, versionId |
| Select Object Content | Expression, ExpressionType, InputSerialization, CSV, AllowQuotedRecordDelimiter, Comments, FieldDelimiter, FileHeaderInfo, QuoteCharacter, QuoteEscapeCharacter, RecordDelimiter, OutputSerialization, CSV |

### Note:

- User-provided metadata header names are stored in lowercase.• User-defined object metadata is supported for the PUT Object and PUT Object-Copy APIs

and is limited to 2 KB in size.

- For bucket tagging, tags that begin with the prefix ntnx: are reserved for internal services and

cannot be used by external clients.

- The maximum size limit of a multipart object is 16 TiB. This limit is different from the standard

S3 API limit of 5 TiB. For more information on objects tagging API, see Buckets and Nutanix Objects Tagging APIs Overview on page 255.For more information on S3 Select API, see Nutanix Objects S3 Select API Overview on page 256. Objects | Nutanix Objects CRUD Operations by Using S3 APIs |

You can generate Presigned URLs for Nutanix Objects and restrict access by specifying an expiration time. The expiration time can exceed seven days. For more information, see Signing and authenticating REST requests in the Amazon Simple Storage Service Developer Guide.

**Table 46: Supported APIs That Use Only Common Headers**

The following table lists the supported APIs that use only Nutanix Objects Common Headers on page 253: Supported APIsGET Bucket lifecycle configurationPUT Bucket lifecycle configurationGET Bucket Object Lock ConfigurationGET Bucket LocationGET Bucket PolicyGET Bucket versioningGET Bucket ACLGET Bucket notification configurationGET Bucket corsGET Bucket websiteGET Bucket replicationPUT Bucket corsPUT Bucket notification configurationPUT Bucket websitePUT Bucket replicationDELETE Bucket corsDELETE Bucket websiteDELETE Bucket LifecycleDELETE Bucket Policy Note: DELETE Bucket Policy is only supported for regular buckets and not federated buckets. DELETE Bucket replicationDELETE ObjectHEAD BucketLIST BucketABORT Multipart Upload

### Nutanix Objects Common Headers

You can use the following headers while making requests: Objects | Nutanix Objects CRUD Operations by Using S3 APIs |

**Table 47: List of Common Headers**

AuthorizationContent-LengthContent-TypeContent-MD5DateExpectHostx-amz-content-sha256x-amz-datex-amz-security-token For more information on common headers, see Common Request Headers in the Amazon Simple Storage Service API Reference Guide.

### Nutanix Objects Unsupported APIs

The following table lists the unsupported S3 API methods:

**Table 48: Unsupported S3 APIs**

Unsupported S3 APIsGET Bucket accelerate configurationGET Bucket analytics configurationGET Bucket encryptionGET Bucket inventory configurationGET Bucket loggingGET Bucket metrics configurationGET Bucket requestPaymentList Bucket Analytics ConfigurationsList Bucket Inventory ConfigurationsList Bucket Metrics ConfigurationsPUT Bucket accelerate configurationPUT Bucket aclPUT Bucket analytics configurationPUT Bucket encryptionPUT Bucket inventory configurationPUT Bucket loggingPUT Bucket metrics configurationDELETE Bucket analytics configuration Objects | Nutanix Objects CRUD Operations by Using S3 APIs |

Unsupported S3 APIsDELETE Bucket inventory configurationDELETE Bucket metrics configurationDELETE Bucket encryptionCopyGET Object ACLPUT Object ACLRestore Object

### Buckets and Nutanix Objects Tagging APIs Overview

A tag is a label that you assign to a bucket or an object, and it consists of a key and a value pair that you can define.The tagging APIs feature allows you to add, retrieve, and remove tags for your buckets and objects.

### Buckets Tagging

- Retrieving bucket metadata also returns the number of tags associated with the bucket, if any.• The maximum number of tags allowed per bucket is 50.• Tag keys can be up to 128 Unicode characters in length, and tag values can be up to 256 Unicode

characters in length.

- Tag keys and values are case-sensitive.

### Nutanix Objects Tagging

- Retrieving objects metadata also returns the number of tags associated with the object, if any.• Tagging is also supported with a few other Object APIs.• The maximum number of tags allowed per object is 10.• Tag keys can be up to 128 Unicode characters in length, and tag values can be up to 256 Unicode

characters in length.

- Tag keys and values are case-sensitive.

### API Operations Supported for Buckets and Nutanix Objects Tagging

Nutanix Objects supports APIs for tagging both buckets and objects. Tag-based bucket lifecycle policy management and object listing are not supported. Bucket Tagging API OperationsNutanix Objects supports the following bucket tagging API operations.

### Note: For bucket tagging, tags starting with prefix ntnx: are reserved for internal services and not allowed

by external clients. Objects | Nutanix Objects CRUD Operations by Using S3 APIs |

| • | PutBucketTagging: Sets the tags for a bucket.The following two scenarios are involved: |
| --- | --- |

- You can add tags to a bucket with no tags associated with it.• You can replace the existing tags associated with the bucket.

| • | GetBucketTagging: Returns the tag set associated with the bucket. |
| --- | --- |
| • | DeleteBucketTagging: Deletes the tags associated with a bucket. |

For more information on bucket tagging, see PutBucketTagging in the Amazon Simple Storage Service API Reference guide. Object Tagging API OperationsNutanix Objects supports the following object tagging API operations.

| • | PUT Object tagging: Replace the tags associated with an object. You can add the tags in the request |
| --- | --- |

body.The following two scenarios are involved:

- You can add tags to an object with no tags associated with it.• You can replace the existing tags associated with an object.

| • | GET Object tagging: Retrieves the tags associated with an object. |
| --- | --- |
| • | DELETE Object tagging: Deletes the tags associated with an object. |

The following Object APIs also support tagging:

- GET Object (returns tag count, if any)• PUT Object• PUT Object-Copy• Initiate Multipart Upload

### Nutanix Objects S3 Select API Overview

S3 Select is a feature to select the partial content of an object and returns the result.With the S3 Select feature, you can only fetch the content of interest. This allows applications to speed up their queries drastically as the size of the sub-set of the object is smaller compared to the actual size of the entire object. S3 Select is supported on objects stored in CSV format and returns the result in CSV format. Note: S3 select does not support compression. The SelectObjectContent API filters the contents of an object located in Nutanix Objects using an SQL statement. In the request, you must specify the SQL expression and data serialization format (CSV) of the object. Nutanix Objects use this format to parse object data into records, and it returns only records that match the specified SQL expression. You must also specify the data serialization format (CSV) for the response. Basic Command SupportNutanix Objects S3 Select feature supports only the SELECT SQL command. Objects | Nutanix Objects CRUD Operations by Using S3 APIs |

The S3 Select query operates on a single object. The primary commands involve data selection and filtering.The following clauses are supported for the SELECT command.

- SELECT list: The SELECT list names the columns, functions, and expressions that you want the query to

return.The following are supported:

- Aggregate operators such as AVG, COUNT, MAX, MIN• Arithmetic operations (For example, +, /)

- FROM clause: Using FROM clause, you can select from arrays or objects within a JSON object. You can

use S3Object as a basic reference.

- WHERE clause: The WHERE clause filters rows based on the condition. A condition is an expression that

has a Boolean result.The following conditions are supported:

- AND, NOT, OR operators• Comparison (For example, <, >=)• Pattern matching (For example, LIKE, %)

- LIMIT clause: The LIMIT clause limits the number of records that you want the query to return based on

the number. The following is the general syntax of a SELECT query. SELECT fields FROM S3Object WHERE condition LIMIT num Note: The specific parameters differ based on the format and schema of the object. For more information on the supported SQL functions, seeSupported SQL Functions by Nutanix Objects S3 Select on page 257.

### RequirementsThe following is the requirement for using S3 SELECT:

- You must have s3:GetObject permission for the object you are querying.

### LimitationsThe following are limitations for using S3 SELECT:

- The maximum length of a SQL expression is 256 KB.• The maximum length of a record in the input or result is 1 MB.• Complex operations like sub-queries or joins are not supported.

### Supported SQL Functions by Nutanix Objects S3 Select

S3 Select supports the following SQL functions. Objects | Nutanix Objects CRUD Operations by Using S3 APIs |

**Table 49: Type of Supported Functions**

| Type of Functions | Functions with Parameters |
| --- | --- |
| Aggregate | avg (X): Returns the average value of all X within a group. String and BLOB values that are not numbers are interpreted as 0.count (X) or count(*): Returns a count of the number of times X is in a group. The count(*) function (with no arguments) returns the total number of rows in the group.max(X): Returns the maximum value of all values in the group. It is the value that is returned last in an ORDER BY on the same column.min(X): Returns the minimum value of all values in the group. This is the first value that appears in an ORDER BY of the column.sum(X): Returns the sum of all values in the group. Objects | Nutanix Objects CRUD Operations by Using S3 APIs | |

| Type of Functions | Functions with Parameters |
| --- | --- |
| Conditional | CASE: A CASE expression is similar to IF-THEN- ELSE. The optional expression that occurs in between the CASE keyword and the first WHEN keyword is called the base expression. There are two forms of the CASE expression: |
| • | A CASE with a base expression: The base expression is evaluated just once, and the result is compared against the evaluation of each WHEN expression from left to right. The result of the CASE expression is the evaluation of the THEN expression that corresponds to the first WHEN expression for which the comparison is true. Or, if none of the WHEN expressions evaluate to a value equal to the base expression, the result of evaluating the ELSE expression, if any. If there is no ELSE expression and none of the WHEN expressions produce a result equal to the base expression, the overall result is NULL. |
| • | A CASE without a base expression: Each WHEN expression is evaluated, and the result is treated as a boolean, starting with the leftmost and continuing to the right. The result of the CASE expression is the evaluation of the THEN expression that corresponds to the first WHEN expression that evaluates to true. Or, if none of the WHEN expressions are evaluated to be true, the result of evaluating the ELSE expression, if any. If there is no ELSE expression and none of the WHEN expressions are true, then the overall result is NULL. A NULL result is considered untrue when evaluating. coalesce(X,Y,...): Returns a copy of the first argument, or NULL if all arguments are NULL. Coalesce() must have at least two arguments.nullif(X,Y): Returns its first argument if the arguments are different and NULL if the arguments are the same. This function searches its arguments from left to right for an argument that defines a collating function and uses that collating function for all string comparisons. Objects | Nutanix Objects CRUD Operations by Using S3 APIs | |

| Type of Functions | Functions with Parameters |
| --- | --- |
| Conversions | CAST: A CAST expression of the form CAST(expr AS type-name) is used to convert the value of expr to a different storage class specified by type-name.The following are the Conversion Processing ways for the different type-name: • Casting a value with no affinity converts into a BLOB. Casting to a BLOB first casts the value to TEXT, then interprets the resulting byte sequence as a BLOB instead of TEXT. • Casting a BLOB value to TEXT, the sequence of bytes that make up the BLOB is interpreted as text encoded using the database encoding. • Casting a BLOB value to a REAL, the value is first converted to TEXT. • Casting a BLOB value to INTEGER, the value is first converted to TEXT. • Casting a TEXT or BLOB value into NUMERIC yields either an INTEGER or a REAL result. Objects | Nutanix Objects CRUD Operations by Using S3 APIs | |

| Type of Functions | Functions with Parameters |
| --- | --- |
| Date | date(time-value, modifier, modifier, ...)time(time-value, modifier, modifier, ...)datetime(time-value, modifier, modifier, ...)All the date and time functions take a time value as an argument. The time value is followed by zero or more modifiers. The date() function returns the date in the YYYY-MM-DD format. The time() function returns the time in the HH:MM:SS format. The datetime() function returns in the YYYY-MM-DDHH:MM:SS format. strftime(format, time-value, modifier, modifier, ...)The strftime() function takes a format string as its first argument. It returns the formatted date according to the format string specified as the first argument. The following is a list of valid strftime() substitutions: • %d - day of month: 00• %f - fractional seconds: SS.SSS• %H - hour: 00-24• %j - day of year: 001-366• %J - Julian day number• %m - month: 01-12• %M - minute: 00-59• %s - seconds since 1970-01-01• %S - seconds: 00-59• %w - day of week 0-6 with Sunday==0• %W - week of year: 00-53• %Y - year: 0000-9999• %% - % Objects | Nutanix Objects CRUD Operations by Using S3 APIs | |

| Type of Functions | Functions with Parameters |
| --- | --- |
| String | length(X): Returns the following based on the value of X: • For a string value, returns the number of characters (not bytes) in X prior to the first NULL character, • For a blob value, returns the number of bytes in the blob. • For a NULL value, returns NULL.• For a numeric value, returns the length of a string representation of X. lower(X): Returns a copy of string X with all ASCII characters converted to lower case. Note: The default lower() function works for ASCII characters only. substring(X,Y,Z): Returns a substring of input string X that begins with the Y-th character and which is Z characters long.Returns the following based on different scenarios: • If Z is omitted, returns all characters through the end of the string X beginning with the Y-th. • If Y is negative, then the first character of the substring is found by counting from the right rather than the left. • If Z is negative, then the Z characters preceding the Y-th character are returned. • If X is a string, then the indices refer to actual UTF-8 characters. • If X is a BLOB, then the indices refer to bytes.ltrim(X,Y): Returns a string formed by removing any and all characters that appear in Y from the left side of X. If the Y argument is omitted, ltrim(X) removes spaces from the left side of X.rtrim(X,Y): Returns a string formed by removing any and all characters that appear in Y from the right side of X. If the Y argument is omitted, rtrim(X) removes spaces from the right side of X.trim(X,Y): Returns a string formed by removing any and all characters that appear in Y from both ends of X. If the Y argument is omitted, trim(X) removes spaces from both ends of X.upper(X): Returns a copy of input string X in which all lower-case ASCII characters are converted to their upper-case equivalent. Objects | Nutanix Objects CRUD Operations by Using S3 APIs | |

### Server-Side Encryption with Customer-Provided Keys Supported APIs

Server-side encryption with customer-provided keys (SSE-C) encrypts objects using Advanced Encryption Standard with Galois Counter Mode (AES-256-GCM), ensuring data integrity and enabling efficient range reads.Each object is encrypted using AES-256-GCM, which generates an authentication tag to ensure data integrity. Decryption requires the same encryption key used during encryption. Nutanix Objects are divided into blocks, and block-level encryption enables efficient range reads without requiring decryption of the entire object.The following table lists the S3 API methods that require additional parameters to support this feature.

**Table 50: S3 API Methods Requiring Additional Parameters for Feature Support**

| Supported S3 APIs | Response Headers/Request Headers |
| --- | --- |
| Head Object | x-amz-server-side-encryption-customer-algorithm: AES256,x-amz-server-side-encryption-customer- key: <Base64-encoded key>, x-amz-server-side- encryption-customer-key-MD5: <Base64-encodedMD5 checksum of Key > |
| Get Object | x-amz-server-side-encryption-customer-algorithm: AES256,x-amz-server-side-encryption-customer- key: <Base64-encoded key>, x-amz-server-side- encryption-customer-key-MD5: <Base64-encodedMD5 checksum of Key > |
| Put Object | x-amz-server-side-encryption-customer-algorithm: AES256,x-amz-server-side-encryption-customer- key: <Base64-encoded key>, x-amz-server-side- encryption-customer-key-MD5: <Base64-encodedMD5 checksum of Key > |
| Copy Object | x-amz-server-side-encryption-customer-algorithm: AES256,x-amz-server-side-encryption-customer- key: <Base64-encoded key>, x-amz-server-side- encryption-customer-key-MD5: <Base64-encodedMD5 checksum of Key > |
| Create Multipart Upload | x-amz-server-side-encryption-customer-algorithm: AES256,x-amz-server-side-encryption-customer- key: <Base64-encoded key>, x-amz-server-side- encryption-customer-key-MD5: <Base64-encodedMD5 checksum of Key > |
| Upload Part | x-amz-server-side-encryption-customer-algorithm: AES256,x-amz-server-side-encryption-customer- key: <Base64-encoded key>, x-amz-server-side- encryption-customer-key-MD5: <Base64-encodedMD5 checksum of Key > |
| Upload Part Copy | x-amz-server-side-encryption-customer-algorithm: AES256,x-amz-server-side-encryption-customer- key: <Base64-encoded key>, x-amz-server-side- encryption-customer-key-MD5: <Base64-encodedMD5 checksum of Key > Objects | Nutanix Objects CRUD Operations by Using S3 APIs | |

### Note: SSE-C is incompatible with buckets with Network File System access, buckets in the Federated

namespace, and buckets with replication configured. An object which is stored with SSE-C, cannot be accessed through S3 Select API. The following table lists the compatibility of SSE-C with other Nutanix Objects features and the expected behavior when used together.

**Table 51: SSE-C Compatibility with other Nutanix Objects Features**

| Feature | Compatibility Details |
| --- | --- |
| Network File System | Network File System can only be enabled at bucket creation. If Network File System access is enabled, an Object Put or Multipart Upload with SSE-C keys fails. |
| Federation | An Object PUT or MULTIPART UPLOAD on a bucket in the federated namespace with SSE-C keys fails. |
| Replication | If replication is configured on a bucket, an Object PUT or MULTIPART UPLOAD with SSE-C keys fails. Additionally, if a bucket contains encrypted objects, configuring replication rules fails. |
| S3 Select | Running S3 Select on an encrypted object results in failure. |

The following table lists alerts related to SSE-C, along with their descriptions.

**Table 52: Alerts**

| Alerts | Description |
| --- | --- |
| DecryptionAuthFailure | Decryption failed due to an AuthTag mismatch |
| EncryptionKeyMismatch | An encryption key mismatch detected. |
| EncryptionParamsNeeded | Missing encryption keys for request. |
| InsecureConnectionWithEncryptionKey | Encryption parameters sent over an insecure connection. Objects | Nutanix Objects CRUD Operations by Using S3 APIs | |

## KAFKA NOTIFICATION SCHEMA

Kafka messages follow a structured JSON schema for object events, including creation, access, and deletion.

**Table 53: Top Level Fields**

| Field | Type Description |
| --- | --- |
| EventName | string Name of the event (s3:ObjectCreated:Put , , s3:ObjectAccessed:Get ). s3:ObjectRemoved:Delete |
| EventType | string Represents the same value as EventName. This field is retained primarily for backward compatibility with older systems or integrations that rely on it. |
| HermesId | string The identifier for the source system or service that generates the event. |
| Key | string The full path to the object within the bucket • For bucket-level events:<bucket name> • For object-level events: <bucket name>/<object name> |
| LogSequence | integer The sequence number that increases monotonically to create an order. |
| Records | array A list of detailed event records (usually one per message). |

**Table 54: Record Fields**

| Field | Type Description |
| --- | --- |
| awsRegion | string The default region configured on the OSS. |
| eventError | string Error message if the event fails (usually empty). |
| eventName | string Same as top-level EventName. |
| eventSource | string Source that is generating the events, currently aws:s3 Objects | Kafka Notification Schema | |

| Field | Type Description |
| --- | --- |
| eventTime | string Timestamp in ISO 8601 format. |
| eventVersion | string Event format version, currently2.0 . |
| qualifiedArnIds | object Maps ARNs to configuration IDs. |
| requestParameters.sourceIPAddressstring | IP address of the requester. |
| responseElements | object We have another element: federationName - Federation name (only in the case of a federated request). |
| s3 | object Details about the bucket and object involved. |
| userIdentity.principalId | string Identity of the user who triggers the event. |

**Table 55: S3 Top Level Fields**

| Field | Type Description |
| --- | --- |
| configurationId | string The type of event can be: • SuccessfulWriteManagementEvent• FailedWriteManagementEvent• DataEvent |
| incarnation | string Internal identifier for component incarnations. Set only for mutation events. |
| mutationSequence | string Sequence number for object mutations. |
| object | object Metadata about the object such as size, content type, and timestamps. |
| ossDomain | array Domain of the object storage service. |
| s3SchemaVersion | string Schema version of the S3 event, currently 1.0. |
| srcBucket | object Metadata about the source bucket. Present only for copy operations. |
| srcObject | object Metadata about the source object. Present only for copy operations. |
| opAttrs | object Additional information about the operation. May or may not be available. Objects | Kafka Notification Schema | |

**Table 56: Bucket Fields**

| Field | Type Description |
| --- | --- |
| name | string The bucket name. |
| ownerIdentity.principalId | string Identity of the bucket owner. |
| bucketId | string Internal ID of the bucket. |
| arn | string Amazon Resource Name (ARN) of the bucket. |

**Table 57: Object Fields**

| Field | Type Description |
| --- | --- |
| key | string The object name. |
| size | integer The object size in bytes. |
| eTag | string The entity tag of the object. |
| tagCount | integer Number of tags associated with the object. |
| versionId | string Version ID of the object if version exists. Empty string if null or absent. |
| createTimestamp | string Timestamp when the object was created. |
| modTimestamp | string Timestamp when the object was last modified. |
| contentType | string Content type of the object. |
| userMetadata | object User metadata key-value pairs associated with the object. |
| sequencer | string Set only if there is a mutation (PUT or DELETE). Same as mutation ID for sequencing. Objects | Kafka Notification Schema | |

## NUTANIX OBJECTS REST ERROR

## RESPONSES

This section provides reference information about Nutanix Objects error responses and codes.When Nutanix Objects request returns an error, the client receives an error response. The format of the error response is API specific; however, all the error responses have common elements.When an error occurs, the header information includes the following:

- Content-Type: application/xml• An appropriate status codeThe following elements are included in REST error responses:

- Code. For more information, see List of Error Codes for Nutanix Objects on page 268.• Error• Message• RequestId• ResourceFor more information on the error responses, see REST Error Responses in the Amazon Simple Storage

Service API Reference Guide.

### List of Error Codes for Nutanix Objects

The following table lists the error codes:

**Table 58: Error Codes for Nutanix Objects**

| Error Code | HTTP Status Code |
| --- | --- |
| AccessDenied | 403 Forbidden |
| AuthorizationHeaderMalformed | 400 Bad Request |
| BadDigest | 400 Bad Request |
| BucketAlreadyExists | 409 Conflict |
| BucketAlreadyOwnedByYou | 409 Conflict |
| BucketNotEmpty | 409 Conflict |
| EntityTooSmall | 400 Bad Request |
| EntityTooLarge | 400 Bad Request |
| IncompleteBody | 400 Bad Request |
| InlineDataTooLarge | 400 Bad Request |
| InternalError | 500 Internal Server Error |
| InvalidAccessKeyId | 403 Forbidden |
| InvalidArgument | 400 Bad Request Objects | Nutanix Objects REST Error Responses | |

| Error Code | HTTP Status Code |
| --- | --- |
| InvalidBucketName | 400 Bad Request |
| InvalidBucketState | 409 Conflict |
| InvalidDigest | 400 Bad Request |
| InvalidLocationConstraint | 400 Bad Request |
| InvalidObjectState | 403 Forbidden |
| InvalidPart | 400 Bad Request |
| InvalidPartOrder | 400 Bad Request |
| InvalidPolicyDocument | 400 Bad Request |
| InvalidRange | 416 Requested Range Not Satisfied |
| InvalidRequest | 400 Bad Request |
| InvalidURI | 400 Bad Request |
| KeyTooLongError | 400 Bad Request |
| MalformedACLError | 400 Bad Request |
| MalformedPOSTRequest | 400 Bad Request |
| MalformedXML | 400 Bad Request |
| MaxMessageLengthExceeded | 400 Bad Request |
| MaxPostPreDataLengthExceededError | 400 Bad Request |
| MetadataTooLarge | 400 Bad Request |
| MethodNotAllowed | 405 Method Not Allowed |
| MissingContentLength | 411 Length Required |
| MissingRequestBodyError | 400 Bad Request |
| NoSuchBucket | 404 Not Found |
| NoSuchBucketPolicy | 404 Not Found |
| NoSuchKey | 404 Not Found |
| NoSuchLifecycleConfiguration | 404 Not Found |
| NoSuchUpload | 404 Not Found |
| InvalidVersion | 404 Not Found |
| NotImplemented | 501 Not Implemented |
| OperationAborted | 409 Conflict |
| PermanentRedirect | 301 Moved Permanent |
| PreconditionFailed | 412 Precondition Failed |
| Redirect | 307 Moved Temporarily |
| RequestIsNotMultiPartContent | 400 Bad Request |
| RequestTimeout | 400 Bad Request |
| RequestTimeTooSkewed | 403 Forbidden |
| SignatureDoesNotMatch | 403 Forbidden Objects | Nutanix Objects REST Error Responses | |

| Error Code | HTTP Status Code |
| --- | --- |
| ServiceUnavailable | 503 Service Unavailable |
| SlowDown | 503 Slow Down |
| TemporaryRedirect | 307 Moved Temporarily |
| UnexpectedContent | 400 Bad Request |

For more information on the error codes, see List of Error Codes in the Amazon Simple Storage Service API Reference Guide. Objects | Nutanix Objects REST Error Responses |

## NUTANIX OBJECTS INTEGRATION WITH

## BACKUP APPLICATIONS

Nutanix Objects is ideal for cost-effective, scale-out storage. It provides a fully distributed, API-accessible storage platform that integrates directly into applications or is used for backup, archiving, and data retention. Nutanix Objects offers you a seamless way to switch from your traditional backup to object store backup. You can also perform multipart uploads. Nutanix Objects supports integration with back up applications such as Commvault, HYCU, Veeam and Veritas.

### Note: Some backup vendors have configurable size limits if larger than 5 TiB VM images need to be

backed up (For example, HYCU). The backup appliance configuration needs to be changed in order to take advantage of the larger size limit in Nutanix Objects. For more information on Commvault Integration, see Commvault with Nutanix guide on the Nutanix Support Portal. Objects | Nutanix Objects Integration with Backup Applications |

