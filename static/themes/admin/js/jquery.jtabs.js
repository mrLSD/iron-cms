jQuery.fn.jtabs = function(options){
	var _options = $.extend({
		tabsSelector: "a",
		activeClass: "active"
	}, options);
	
	return $(this).each(function() {
		var tabLinks = $(this).find(_options.tabsSelector);
		var panels = [];
		
		tabLinks.each(function(e, elem){
			panels.push($($(elem).attr("href")));
		});
		
		hideAllPanels();
		
		tabLinks.bind("click", function(e){
			e.preventDefault;
			//if($(this).hasClass(_options.activeClass)) return false;
			tabLinks.removeClass(_options.activeClass);
			$(this).addClass(_options.activeClass);
			hideAllPanels();
			$($(this).attr("href")).css({
				"height": "auto",
				"opacity": 1
			});
			return false;
		});
		
		tabLinks.filter("." + _options.activeClass).trigger("click");
		
		function hideAllPanels(){
			$(panels).each(function(e, elem){
				$(elem).css({
					"height": "1px",
					"opacity": 0.01
				});
			});
		}
	});
}