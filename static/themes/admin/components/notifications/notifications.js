//------------------------------------------------------------------------------
(function(window, undefined){
	var document = window.document;
	function note(){
		var self = this;
		self.settings = {
			'dependencies' : [
				'http://ajax.googleapis.com/ajax/libs/jquery/1/jquery.min.js'
			],
			'spacing' : {
				'x' : 10,
				'y' : 10
			},
			'initialized' : false
		};
		self.notes = [];
		self.queued = [];
		
		/*** UTILITY FUNCTIONS ***/
		var load_dependencies = function(){
			var depends = self.settings.dependencies;
			var total = depends.length; self.settings.dependencies_loaded = 0;
			var func = (function(d, t, odl){
				return function(){
					d++;
					if(d === t){
						odl(); 
					}
				};
			})(self.settings.dependencies_loaded, total, on_dependencies_loaded);
			
			for(var i = 0; i < total; i++){
				depends[i] += '?time=' + (new Date()).getTime();
				get_script(depends[i], func, true);
			}
		};
		var add_event = function(evt, obj, func){
			if(window.addEventListener){
				obj.addEventListener(evt, func, false);
			}
			else if(document.attachEvent){
				obj.attachEvent('on' + evt, func);
			}
		};
		var get_script = function(url, callback, remove){
			var head = document.getElementsByTagName('head')[0];
			var script = document.createElement('script');
			script.type = 'text/javascript';
			script.src = url;
			var func = (function(h, s, c, r){
				return function(){
					if(c !== undefined){
						c();
					}
					if(r === undefined || r === true){
						h.removeChild(s);
					}
				};
			})(head, script, callback, remove);
			script.onload = func;
			script.onreadystatechange = (function(f, s){
				return function(){
					if(s.readyState === 'complete' || s.readyState === 'loaded'){
						f();
					}
				};
			})(func, script);
			//head.appendChild(script);
		};
		var on_dependencies_loaded = function(){
			init();
		};
		var get_style = function(style, obj){
			if(obj.currentStyle){
				return obj.currentStyle[style];
			}
			else{
				return document.defaultView.getComputedStyle(obj, null).getPropertyValue(style);
			}
		};
		var dom = function(){
			var is_ready = false;
			var funcs = [];
			var self = this;
			var scroll_check = function(){
				if(is_ready){
					return;
				}
				try{
					document.documentElement.doScroll("left");
				}
				catch(error){
					setTimeout((function(sc){
						return function(){
							sc();
						};
					})(scroll_check), 1);
					return;
				}
				ready();
			};
			var ready = function(){
				if(is_ready){
					return;
				}
				for(var i = 0, len = funcs.length; i < len; i++){
					funcs[i]();
				}
				is_ready = true;
			};
			self.ready = function(func){
				funcs[funcs.length] = func;
			};
			self.listen = function(){
				if(document.readyState === "complete"){
					return on_ready();
				}
				if(document.addEventListener){
					add_event('DOMContentLoaded', document, (function(ready){
						return function(){
							ready();
						};
					})(ready)); 
					add_event('load', window, (function(ready){
						return function(){
							ready();
						};
					})(ready));
				}
				else if(document.attachEvent){
					add_event('load', window, (function(ready){
						return function(){
							ready();
						};
					})(ready));
					add_event('readystatechange', document, (function(ready){
						return function(){
							ready();
						};
					})(ready));
					var top = false; 
					try{
						top = window.frameset === null;
					}
					catch(e){};
					if(document.documentElement.doScroll && top){
						scroll_check();
					}
				}
			};
		};
		var set_class = function(class_name, obj){
			obj.className = class_name;
		};
		var get_scroll_pos = function(){
			var scroll = document.body.scrollTop;
			if(scroll === 0){
				if(window.pageYOffset){
					scroll = window.pageYOffset; 
				}
				else{
					scroll = (document.body.parentElement) ? document.body.parentElement.scrollTop : 0;
				}
			}
			return scroll;
		};		
		var get_viewport = function(){
			var w; var h;
			if(typeof window.innerWidth !== 'undefined'){
				w = window.innerWidth;
				h = window.innerHeight;
			}
			else if(typeof document.documentElement !== 'undefined' && typeof document.documentElement.clientWidth !== 'undefined' && document.documentElement.clientWidth !== 0){
				w = document.documentElement.clientWidth;
				h = document.documentElement.clientHeight; 
			}
			else{
				w = document.getElementsByTagName('body')[0].clientWidth;
				h = document.getElementsByTagName('body')[0].clientHeight;
			}
			return [w, h];
		};
		var objtostr = function(obj){
			var str = '';
			for(var i in obj){
				str += i + ': ' + obj[i] + '; ';
			}
			return str;
		};
		
		var init = function(){
			self.settings.initialized = true;
			add_event('resize', window, (function(ap){
				return function(e){
					ap();
				};
			})(adjust_pos));
			
			fire_queued_notes();
		};
		var fire_queued_notes = function(){
			if(self.queued.length === 0){
				return;
			}
			var tmp;
			for(var i = 0, len = self.queued.length; i < len; i++){
				tmp = self.queued[i];
				self.show(tmp[0], tmp[1], tmp[2]);
			}
		};
		self.show = function(message, type, settings){
			//console.log("show");
			/*if(self.settings.initialized === false){
				self.queued.push([message, type, settings]);
				return;
			}*/
		
			type = (type !== undefined || type === '') ? type : 'info';
			
			if(settings === undefined){
				settings = {'anchor' : 'br', 'width' : 250, 'autoremove' : true, 'lifetime' : 4000, 'closeable' : true};
			}
			else{
				settings.anchor = (settings.anchor !== undefined) ? settings.anchor : 'br';
				settings.width = (settings.width !== undefined) ? settings.width : 250;
				settings.autoremove = (settings.autoremove !== undefined) ? settings.autoremove : true;
				settings.lifetime = (settings.lifetime !== undefined) ? settings.lifetime : 4000;
				settings.closeable = (settings.closeable !== undefined) ? settings.closeable : true;
			}
			
			settings.type = type;
			
			var note = create_note(type, message, settings);
			var vp = get_viewport();
			note.settings = settings;
			
			self.notes.push(note);

			document.body.appendChild(note);
			anchor(note, type, settings.anchor);
			adjust_pos();
			
			var remove = (function(hide, n){
				return function(){
					hide(n);
				};
			})(self.hide, note);
			
			var start_animation = (function(r, note){
				return function(){
					$(note).animate({'opacity' : '0.00'}, 3000, r);
				};
			})(remove, note);
			
			if(settings.autoremove === true){
				setTimeout(start_animation, settings.lifetime);
			}
		};
		self.hide = function(note){
			var tmp;
			for(var i = 0, len = self.notes.length; i < len; i++){
				tmp = self.notes[i];
				if(note === tmp){
					self.notes.splice(i, 1);
					break;
				}
			}
			
			document.body.removeChild(note);
			for(var i = 0, len = self.notes.length; i < len; i++){
				tmp = self.notes[i];
				if(tmp.settings.anchor === note.settings.anchor){
					anchor(tmp, tmp.settings.type, tmp.settings.anchor);
				}
			}
		};
		var adjust_pos = function(){
			var vp = get_viewport();
			var tmp, pos;
			for(var i = 0, len = self.notes.length; i < len; i++){
				tmp = self.notes[i];
				pos = anchor(tmp, tmp.settings.type, tmp.settings.anchor);
			}
		};
		var create_note = function(type, msg, settings){
			var d = document.createElement('div');
			var icon = document.createElement('div');
			icon.style.cssText = 'float: left; display: inline;';
			d.appendChild(icon);
			var text = document.createElement('div');
			text.style.cssText = 'float: left; display: inline;';
			text.innerHTML = msg;
			d.appendChild(text);
			var clear = document.createElement('div');
			clear.style.cssText = 'clear: both;';
			d.appendChild(clear);
			if(settings.autoremove === false && settings.closeable === true){
				var close = document.createElement('div');
				set_class('note_close', close);
				add_event('click', close, (function(h, n){
					return function(){
						h(n);
					};
				})(self.hide, d));
				d.appendChild(close);
			}
			
			if(type ==='info'){
				set_class('note_container note_info', d);
				set_class('note_icon_info', icon);
				set_class('note_text_info', text);
			}
			else if(type === 'error'){
				set_class('note_container note_error', d);
				set_class('note_icon_error', icon);
				set_class('note_text_error', text);
			}
			else if(type === 'warning'){
				set_class('note_container note_warning', d);
				set_class('note_icon_warning', icon);
				set_class('note_text_warning', text);
			}
			else if(type === 'confirm'){
				set_class('note_container note_confirm', d);
				set_class('note_icon_confirm', icon);
				set_class('note_text_confirm', text);
			}
			
			d.style.position = 'absolute';
			d.style.width = settings.width + 'px';
			text.style.maxWidth = (settings.width - 50) + 'px';
			
			return d;
		};
		var calculate_offsets = function(elem, type, area){
			var elems = [], tmp, y = 0;
			for(var i = 0, len = self.notes.length; i < len; i++){
				tmp = self.notes[i];
				if(tmp === elem){
					break;
				}
				if(tmp.settings.anchor === area){
					elems.push(tmp);
				}
			}
			
			if(elems.length === 0){
				return y;
			}
			
			var yspacing = self.settings.spacing.y;
			
			for(var i = 0, len = elems.length; i < len; i++){
				tmp = elems[i];
				if(area === 'tl' || area === 'tr' || area === 'tm'){
					y += tmp.offsetHeight + yspacing;
				}
				else{
					y -= tmp.offsetHeight + yspacing;
				}
			}
			return y;
		};
		var anchor = function(note, type, area){
			var vp = get_viewport();
			var sp = get_scroll_pos();
			var x = 0, y = 0;
			var offsets = calculate_offsets(note, type, area);
			y += offsets;
			
			var spacing = self.settings.spacing;
			
			if(area === 'br'){
				x += vp[0] - spacing.x - note.offsetWidth;
				x -= (vertical_bar_visible() === true) ? 20 : 0;
				y += sp + vp[1] - spacing.y - note.offsetHeight;
			}
			else if(area === 'tr'){
				x += vp[0] - spacing.x - note.offsetWidth;
				x -= (vertical_bar_visible() === true) ? 20 : 0;
				y += sp + spacing.y;
			}
			else if(area === 'bl'){
				x += spacing.x;
				y += sp + vp[1] - spacing.y - note.offsetHeight;
			}
			else if(area === 'tl'){
				x += spacing.x;
				y += spacing.y;
			}
			else{
				x += vp[0] - spacing.x - note.offsetWidth;
				x -= (vertical_bar_visible() === true) ? 20 : 0;
				y += sp + vp[1] - spacing.y - note.offsetHeight;
			}
			
			note.style.left = x + 'px';
			note.style.top = y + 'px';
		};
		var vertical_bar_visible = function(){
			return ($(document).height() > $(window).height()) ? true : false;
		};
		
		/*** DOM READY EVENTS INSTANTIATION ***/
		self.dom = new dom();
		self.dom.listen();
		self.dom.ready((function(ld){
			return function(){
				ld(); 
			};
		})(load_dependencies));
	};
	window.$note = new note();
})(window);
//------------------------------------------------------------------------------