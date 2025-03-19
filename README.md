# Strava SDK

These are auto generated SDKs for [Strava API]. The spec comes from [developers.strava.com/swagger/swagger.json][spec]

## `openapi-codegen`

We need to use `--skip-validate-spec` since there are some issues with the spec.

```sh
Errors:
        -attribute paths.'/routes/{id}/export_gpx'(get).responses.200.content is unexpected
        -attribute paths.'/routes/{id}/export_tcx'(get).responses.200.content is unexpected
```

```sh
› brew install openapi-generator
› openapi-generator generate -i swagger.json -g rust -o openapi-codegen --skip-validate-spec --additional-properties=useSingleRequestParameter=true
```

By default this also doesn't generate 100% valid code so we need a few fixes:

- `update_logged_in_athlete` - `weight` parameter missing from form data
- `get_routes_by_athlete_id` - parameters is missing the `id`

They can both be fixed with the following patch (this is already committed in
this repository):

```path
diff --git a/swagger.json b/swagger.json
index 3053730..fa55356 100644
--- a/swagger.json
+++ b/swagger.json
@@ -171,7 +171,7 @@
         "parameters": [
           {
             "name": "weight",
-            "in": "path",
+            "in": "formData",
             "description": "The weight of the athlete in kilograms.",
             "required": true,
             "type": "number",
@@ -2268,6 +2268,14 @@
         "summary": "List Athlete Routes",
         "description": "Returns a list of the routes created by the authenticated athlete. Private routes are filtered out unless requested by a token with read_all scope.",
         "parameters": [
+          {
+            "name": "id",
+            "in": "path",
+            "description": "The identifier of the athlete. Must match the authenticated athlete.",
+            "required": true,
+            "type": "integer",
+            "format": "int64"
+          },
           {
             "$ref": "#/parameters/page"
           },
```

## `swagger-codegen`

The recommended way according to the [docs] is to use `swagger-codegen` (v2)
instead of `openapi-codegen` but since the latest version of `openapi-codegen`
is working that's what I'm using.

If you want to generate code with `swagger-codegen`, this would be the way.

```sh
› brew install swagger-codegen@2
› swagger-codegen generate --input-spec swagger.json --lang rust --output swagger-codegen
```

## Authentication

Strava uses OAuth and before I tried a codegen I found [strava-client-rs]. I
didn't end up using that library since it only had a subset of the API
implemented and also some types wasn't fully correct. However the tool had a
fully working OAuth implementation including spinning up a server for the
callback. To avoid rolling that myself it's added as a dependency and used to
generate tokens here.

[Strava API]: https://developers.strava.com/docs/reference/
[docs]: https://developers.strava.com/docs/
[spec]: https://developers.strava.com/swagger/swagger.json
[strava-client-rs]: https://github.com/qgriffith/strava-client-rs
