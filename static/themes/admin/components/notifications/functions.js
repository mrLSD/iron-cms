//------------------------------------------------------------------------------
(function(window, undefined){
	var document = window.document;
	function demo(){
		var self = this;
		self.demo_text = 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec sit amet odio lorem, et tempus dui. Nulla enim nisl, lobortis a semper nec, consequat eu ipsum. Sed a vestibulum velit. Curabitur at eros sapien, quis feugiat mauris.';
		self.demo_text2 = 'These notifications can even fit ordered lists!<ul><li>List Item 1</li><li>List Item 2</li><li>List Item 3</li></ul>';
		self.demo_text3 = 'Fit your social networking icons into a notification!<br/><br/><div class="facebook"></div><div class="twitter"></div><div class="linkedin"></div><div class="youtube"></div><div class="clear"></div>';
		self.demo_text4 = 'Notifications allows you to do cool stuff like embed youtube videos<br/><br/><iframe title="YouTube video player" width="200" height="200" src="http://www.youtube.com/embed/YIW5oo-8NYw" frameborder="0" allowfullscreen></iframe>';
		
		var _$ = function(div){ var d = document.getElementById(div); if(d){ return d; } else{ return false; } };
		var add_event = function(evt, obj, func){ if(window.addEventListener){ obj.addEventListener(evt, func, false); } else if(document.attachEvent){ obj.attachEvent('on' + evt, func); } };
		
		self.init = function(){
			$note.show(self.demo_text, 'warning', {'anchor' : 'br', 'width' : 250, 'autoremove' : false});
			$note.show(self.demo_text2, 'info', {'anchor' : 'tr', 'width' : 250, 'autoremove' : false});
			$note.show(self.demo_text3, 'error', {'anchor' : 'tl', 'width' : 250, 'autoremove' : false});
			$note.show(self.demo_text4, 'confirm', {'anchor' : 'bl', 'width' : 250, 'autoremove' : false});
		};
		self.show_features = function(){
			_$('examples').style.display = 'none';
			_$('howto').style.display = 'none';
			_$('features').style.display = 'block';
			_$('contact').style.display = 'none';
		};
		self.show_howto = function(){
			_$('examples').style.display = 'none';
			_$('howto').style.display = 'block';
			_$('features').style.display = 'none';
			_$('contact').style.display = 'none';
		};
		self.show_examples = function(){
			_$('examples').style.display = 'block';
			_$('howto').style.display = 'none';
			_$('features').style.display = 'none';
			_$('contact').style.display = 'none';
		};
		self.show_contact = function(){
			_$('examples').style.display = 'none';
			_$('howto').style.display = 'none';
			_$('features').style.display = 'none';
			_$('contact').style.display = 'block';
		};
	};
	window.$demo = new demo();
})(window);
//------------------------------------------------------------------------------