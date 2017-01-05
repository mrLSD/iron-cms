/*
Name: AttentionBox
Author: Denon Studio
URL: http://codecanyon.net/user/denonstudio/
License: http://codecanyon.net/wiki/support/legal-terms/licensing-terms/
Copyright: 2010 Denon Studio
*/

var AttentionBox = new function(){
	
	this.container   = null;
	this.modal       = null;
	this.timerHandle = null;
	this.callback    = null;
	this.isShaking   = false;
	this.isIE6	     = (jQuery.browser.msie && jQuery.browser.version == "6.0");
	var self         = this;
	
	this.showMessage = function(message, options)
	{
		if (this.container != null)
		{
			this.shake();
			return;
		}

		if (options && options.modal)
		{
			this.modal = this.generatModalPanel((options) ? options.modalcolor : null);
			jQuery(document.body).append(this.modal);
		}

		this.container = jQuery(document.createElement("div")).attr("class", "attention-box");
		var inputsrow  = this.generateInputsRow((options && options.inputs) ? options.inputs : null);
		var messagerow = jQuery(document.createElement("div")).attr("class", "message").append(message);
		var buttonsrow = this.generateButtonsRow((options && options.buttons) ? options.buttons : [{caption : "Okay"}]);
		self.callback  = (options) ? options.callback : null;

		if (this.isIE6)
		{
			messagerow.addClass("bubbleie6");
			this.container.addClass("attention-box-ie6");
		}
		else
		{
			messagerow.addClass("bubble");
		}			

		this.container.append(messagerow).append(inputsrow).append(buttonsrow).hide();
		jQuery(document.body).append(this.container);

		if (this.modal)
		{
			this.modal.fadeTo(400, 0.5);
		}
			
		this.center(this);	
		this.container.fadeIn(200, this.bindListeners);
	};
	
	this.bindListeners = function()
	{
		jQuery(window  ).bind("scroll", self.onResize);
		jQuery(window  ).bind("resize", self.onResize);
		jQuery(document).bind("keyup" , self.onKeyup);
	};
	
	this.unbindListeners = function()
	{
		jQuery(window  ).unbind("scroll", self.onResize);
		jQuery(window  ).unbind("resize", self.onResize);
		jQuery(document).unbind("keyup" , self.onKeyup );
	};

	this.onResize = function()
	{
		if (self.container)
			self.center(self, true);			
			
		if (self.modal)
			self.modal.css("height", jQuery("body").outerHeight());
	};
	
	this.onKeyup = function(e)
	{
		if (self.container && self.container != null && e.keyCode == 27)
		{
			self.closeWindow(e, true);
		}
		e.result = true;
		return true;
	};

	this.shake = function()
	{
		if (this.container != null && !this.isShaking)
		{
			self.isShaking = true;
			this.container.effect("shake", { times: 2, direction: "up" }, 100, function(){ self.isShaking = false; });
			return;
		}
	};
	
	this.center = function(self, animate)
	{
		var top  = ( jQuery(window).height() - self.container.height()) / 2 + jQuery(window).scrollTop () + "px";
		var left = ( jQuery(window).width () - self.container.width ()) / 2 + jQuery(window).scrollLeft() + "px";
		
		if (animate)
		{
			clearTimeout(this.timerHandle);
			this.timerHandle = setTimeout(function()
			{ 
				self.container.animate({ "top": top, "left": left }, 600, "easeInOutExpo");
			}, 300);
		}
		else
		{	
			self.container.css("top", top);
    		self.container.css("left", left);
    	}
	};

	this.generateButtonsRow = function(buttons)
	{
		var container = jQuery(document.createElement("div")).attr("class", "buttons");

		for (var i = buttons.length - 1; i >= 0; i--)
		{
			var item   = buttons[i];
			var button = jQuery(document.createElement("button")).text(item.caption);
			
			if (item.important && item.important == true)
				button.attr("class", "important");
				
			if (this.isIE6)
				button.addClass("ie6IsCrap");

			if (item.cancel && item.cancel == true)	
				button.attr("cancel", "yes");
			
			button.bind("click", self.closeWindow);
			container.append(button);
		}
			
		return container;
	};
	
	this.closeWindow = function(e, isCancelled)
	{
		isCancelled = (isCancelled || jQuery(e.target).attr("cancel") == "yes");
		
		if (!isCancelled)
		{
			var requiredAndEmpty = self.container.find("input[value=][req=yes]");
			if (requiredAndEmpty.length != 0)
			{
				jQuery(requiredAndEmpty).effect("highlight", {color: "#FFA3A3"}, 2000);
				return;
			}
	
			var textboxes = self.container.find("input");
			var inputs    = [];
			
			if (textboxes.length > 0)
			{
				for (var i = 0; i < textboxes.length; i++)
				{
					var element = jQuery(textboxes[i]);
					inputs[i] = { caption: element.attr("rel"), value: element.val() };
				}
			}
		}

		if (self.modal)
			self.modal.fadeOut(200);

		self.container.fadeOut(200, function(){
			self.container.detach();
			self.container = null;
			
			self.unbindListeners();

			if (self.modal)
			{
				self.modal.detach();
				self.modal = null;
			}

			if (self.callback)
			{
				var c = self.callback;
				self.callback = null;
				window.setTimeout(function()
				{
					if (isCancelled)
					{
						c("CANCELLED");
					}
					else
					{
						c(jQuery(e.target).text(), inputs);
					}
				}, 0);
			}
		});
	};
	
	this.generateInputsRow = function(inputs)
	{
		if (!inputs || inputs.length == 0)
			return;
			
		var container = jQuery(document.createElement("div")).attr("class","input-container");

		for (var i = 0; i < inputs.length; i++)
		{
			var item = inputs[i];
			
			var label = jQuery(document.createElement("label")).attr("for", i).append(item.caption);
			var input = jQuery(document.createElement("input")).attr({"type": "text", "name" : i, "rel" : item.caption });

			if (item.required && item.required == true)
				input.attr("req","yes");
			
			if (item.value)
				input.attr("value", item.value);

			container.append(label);

			if (item.error)
				label.append("<span class=\"error\">" + item.error + "</span>");		

			container.append(input.wrap("<div></div>").parent());		
			
		}	
		
		return container;
	};
	
	this.generatModalPanel = function(modalcolor)
	{
		var result = jQuery(document.createElement("div")).attr("class", "attention-box-modal").fadeTo(0,0).css("height", jQuery(window).height());

		if (modalcolor)
			result.css({ "background-color": modalcolor});
			
		if (this.isIE6)	
		{
			result.css("position", "absolute");
			result.css("height", jQuery("body").outerHeight());
		}

		return result;
	};
};