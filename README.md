# reMarkable Manager

The purpose of this tool is to manage my [reMarkable 2 tablet](https://remarkable.com/store/remarkable-2/). The reMarkable tablet allows users to `ssh` in and perform modifications to the system. However, this is not officially supported and those modifications can be rolled back when the a new version of the software is installed.

This tool focuses on enabling custom templates. Template customization is documented via a [community wiki](https://remarkablewiki.com/tips/templates?s[]=template) and is done by copying over `.png`, `.svg`, and `templates.json` files to the device and then restarting a systemd service.

Given a configuration, this tool will:

1. Back up the existing `/usr/share/remarkable/templates/` folder from the reMarkable device to the computer running the tool
2. Modify the `templates.json` to include the user's custom template information
3. Copy the custom templates and the new `templates.json` file to the reMarkable device
4. Restart the systemd service that manages the templates

This tool can also be used to restore an existing backup. From initial testing, an invalid `templates.json` does not cause problems that prevent `ssh` access to the device, so enabling a restore-from-backup function is preferable to a factory-reset.
