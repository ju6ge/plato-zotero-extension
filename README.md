Zotero Plato Sync
=================

This program is an extension to the plato reader. It enables reading papers managed with zotero with [plato](https://github.com/baskerville/plato).

## Usage

This extension queries the Zotero Api for Items with the Tag `plato-read`. These items are downloaded to the plato device.

## Configuration

The configuration of the extension lives in the `ZoteroSettings.toml` file:

``` toml
zotero_id = "123456"
zotero_key = "ZoteroApiKey"
webdav_user = "user"
webdav_password = "password"
webdav_url = "https://<nextcloud_url>/remote.php/files/<user>/<subdir>"

```
