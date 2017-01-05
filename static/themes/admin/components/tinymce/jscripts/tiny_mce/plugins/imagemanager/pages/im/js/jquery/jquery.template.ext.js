/**
 * $Id: jquery.template.ext.js 453 2008-10-14 12:24:41Z spocke $
 *
 * @author Moxiecode
 * @copyright Copyright © 2004-2008, Moxiecode Systems AB, All rights reserved.
 */

(function($){
	$.template.regx.standard = $.template.regx.ext;

	$.templateFromScript = function(el, opts) {
		return $.template($.trim($.translate($(el).html().replace(/(<!\[CDATA\[|\]\]>)/gi, ''))), opts);
	};

	// Add template helpers
	$.extend($.template.helpers, {
		sizefix : function(v) {
			v = parseInt(v);

			if (isNaN(v) || v == -1)
				return '';

			// MB
			if (v > 1048576)
				return Math.round(v / 1048576, 1) + " MB";

			// KB
			if (v > 1024)
				return Math.round(v / 1024, 1) + " KB";

			return v + " b";
		},

		encodeURIComponent : function(v) {
			return encodeURIComponent(v);
		}
	});
})(jQuery);