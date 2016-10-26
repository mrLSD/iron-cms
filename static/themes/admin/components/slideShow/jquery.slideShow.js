/* 
* jQuery slideShow v1.2.0
*/

jQuery.fn.slideShow = function(_options){    
	// defaults options	    
	var _options = jQuery.extend({		
		slideEl:'div.slide',		
		linkNext:'a.next',		
		linkPrev:'a.prev',		
		linkPause:'a.pause',		
		numElement:'div.slideNav li a',		
		duration:500,		
		autoSlideShow:false,		
		switchTime:3000,
		noCicle:false,
		disableClass:'no-active',
		event:'click',		
		currentEl:'span.cur',		
		allEl:'span.all'    
	},_options);
    return this.each(function(){
	    var _THIS = jQuery(this);
		var _slideEl = $(_options.slideEl, _THIS);
		
		var _linkNext = $(_options.linkNext, _THIS);
		var _linkPrev = $(_options.linkPrev, _THIS);
		var _linkPause = $(_options.linkPause, _THIS);
		var _numElement = $(_options.numElement, _THIS);
		var _currentEl = $(_options.currentEl, _THIS);
		var _allEl = $(_options.allEl, _THIS)

		var _duration = _options.duration;
		var _switchTime = _options.switchTime;
		var _numElActive, _timer = false, _hover = false, _current = 0, _next = 0, _pause = true;
		
		if (!_slideEl.filter('.active').length) {
			_slideEl.eq(0).addClass('active');
			_current = 0;
		} else {
			_current = _slideEl.index(_slideEl.filter('.active'));
		}
		_slideEl.not(".active").hide();
		
		if ($(_numElement).length && _options.numElement) activeNumEl();
		if (_options.autoSlideShow) {
			_pause = false;
			_timer = setTimeout(function(){nextEl()},_switchTime);
			if (_linkPause.length && _options.linkPause) _linkPause.addClass('play')	
			contentHover();
		}
		if (_options.currentEl && _currentEl.length) {
			_allEl.html(_slideEl.length);
			currentNum();
		}
		if (_linkNext.length && _options.linkNext) {
			_linkNext.click(function(){
				nextEl();
				return false;
			});
		}
		if (_options.noCicle) {
			_linkPrev.addClass('prev-'+_options.disableClass);
		}
		if (_linkPrev.length && _options.linkPrev) {	
			_linkPrev.click(function(){
				_linkNext.removeClass('next-'+_options.disableClass);
				if (!_slideEl.is(':animated') && !$(this).hasClass('prev-'+_options.disableClass)) {
					if (_timer) clearTimeout(_timer);
					_next = _current-1;
					if (_next < 0) _next = _slideEl.length-1;
					if (_options.noCicle && _next-1 < 0) {
						$(this).addClass('prev-'+_options.disableClass);
					}
					fadeElement();
					activeNumEl();
					pauseCode();
				}
				return false;
			});
		}
		if (_numElement.length && _options.numElement) {
			_numElement.bind(_options.event, function(){
				if (!_slideEl.is(':animated')) {
					_next = _numElement.index($(this));
					_linkNext.removeClass('next-'+_options.disableClass);
					_linkPrev.removeClass('prev-'+_options.disableClass);
					if (_options.noCicle && _next+1 >= _slideEl.length) _linkNext.addClass('next-'+_options.disableClass);
					if (_options.noCicle && _next-1 < 0) _linkPrev.addClass('prev-'+_options.disableClass);
					if (_timer) clearTimeout(_timer);
					if (!_slideEl.eq(_next).hasClass("active")){
						fadeElement();
						activeNumEl();
						pauseCode();
					}
				}
				return false;
			});
		}
		if (_linkPause.length && _options.linkPause) {
			_linkPause.click(function(){
				if (!_pause) {
					$(this).removeClass('play');
					clearTimeout(_timer);
					_pause = true;
				} else {
					$(this).addClass('play');
					_timer = setTimeout(function(){nextEl()},_switchTime);
					_pause = false;
				}
				return false;
			});
		}
		function fadeElement(){
			if (_current != _next) {
				_slideEl.removeClass('active');
				_slideEl.eq(_current).addClass('hide');
				_slideEl.eq(_next).fadeIn(_duration, function(){
					_slideEl.filter('.hide').hide().removeClass('hide');
				}).addClass('active');
				_current = _next;
				if (_options.currentEl && _currentEl.length) currentNum();
			}
		};
		function currentNum() {
			_currentEl.html(_current+1);
		}
		function activeNumEl() {
			_numElement.parent().removeClass("active");
			_numElement.eq(_current).parent().addClass("active");
		};
		function nextEl(){
			_linkPrev.removeClass('prev-'+_options.disableClass);
			if (!_slideEl.is(':animated') && !_linkNext.hasClass('next-'+_options.disableClass)) {
				if (_timer) clearTimeout(_timer);
				_next = _current+1;
				if (_next == _slideEl.length) _next = 0;
				if (_options.noCicle && _next+1 >= _slideEl.length) {
					_linkNext.addClass('next-'+_options.disableClass);
				}
				fadeElement();
				activeNumEl();
				pauseCode();
			}
		};
		function pauseCode(){
			if (!_pause) {
				if (_linkPause.length && _options.linkPause) {
					if (_linkPause.hasClass('play')) {
						_timer = setTimeout(function(){nextEl()},_switchTime);
					}					
				} else {
					_timer = setTimeout(function(){nextEl()},_switchTime);
				}
			}
		}
		function contentHover() {
			_hover = true;
			_slideEl.mouseenter(function() {
				if (_timer) clearTimeout(_timer);
			}).mouseleave(function(){
				pauseCode();
			});			
		}
    });
}


