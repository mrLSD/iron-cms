<?php
/**
 * JoomlaAuthenticatorImpl.php
 *
 * @package MCImageManager.authenicators
 * @author Moxiecode
 * @copyright Copyright © 2005-2006, Moxiecode Systems AB, All rights reserved.
 */

// Include Joomla bootstrap logic
@session_destroy();
$mcOldCWD = getcwd();
chdir($basepath . "../../../../../../../../");
define('_VALID_MOS', 1);
define('RG_EMULATION', 0);

require_once('configuration.php');
require_once('includes/joomla.php');

// Try this
$mainframe = new mosMainFrame($database, $option, '.');
$mainframe->initSession();
$mamboUser = $mainframe->getUser();

// Try that
if ($mamboUser->id == 0) {
	session_name(md5($mosConfig_live_site));
	session_start();

	$mamboUser = new mosUser($database);
	$mamboUser->id = intval(mosGetParam($_SESSION, 'session_user_id', ''));
	$mamboUser->username = strval(mosGetParam($_SESSION, 'session_username', ''));
	$mamboUser->usertype = strval(mosGetParam($_SESSION, 'session_usertype', '')); 
}

chdir($mcOldCWD);

/**
 * This class is a Drupal CMS authenticator implementation.
 *
 * @package MCImageManager.Authenticators
 */
class Moxiecode_JoomlaAuthenticator extends Moxiecode_ManagerPlugin {
    /**#@+
	 * @access public
	 */

	/**
	 * Main constructor.
	 */
	function Moxiecode_JoomlaAuthenticator() {
	}

	function onAuthenticate(&$man) {
		global $mamboUser;

		$config =& $man->getConfig();

		// Not logged in
		if ($mamboUser->id == 0)
			return false;

		// Replace ${user} in all config values
		foreach ($config as $key => $value) {
			// Skip replaceing {$user} in true/false stuff
			if ($value === true || $value === false)
				continue;

			$value = str_replace('${user}', $mamboUser->username, $value);
			$config[$key] = $value;
		}

		// Try create rootpath
		$rootPath = $man->toAbsPath($config['filesystem.rootpath']);
		$rootPathItems = explode(';', $rootPath);
		$rootPathItems = explode('=', $rootPathItems[0]);

		if (count($rootPathItems) > 1)
			$rootPath = $rootPathItems[1];
		else
			$rootPath = $rootPathItems[0];

		if (!file_exists($rootPath))
			@mkdir($rootPath);

		// Is one of the valid user names
		return preg_match($config['JoomlaAuthenticator.valid_users'], $mamboUser->username);
	}

	/**#@-*/
}

// Add plugin to MCManager
$man->registerPlugin("JoomlaAuthenticator", new Moxiecode_JoomlaAuthenticator());

?>