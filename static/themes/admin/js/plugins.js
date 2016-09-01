window.log = function(){
  log.history = log.history || [];  
  log.history.push(arguments);
  arguments.callee = arguments.callee.caller;  
  if(this.console) console.log( Array.prototype.slice.call(arguments) );
};
(function(b){function c(){}for(var d="assert,count,debug,dir,dirxml,error,exception,group,groupCollapsed,groupEnd,info,log,markTimeline,profile,profileEnd,time,timeEnd,trace,warn".split(","),a;a=d.pop();)b[a]=b[a]||c})(window.console=window.console||{});

/**
 * Image crop plugin
 * @autor Ruslan Kesheshyan aka M.O.C.K. ruslan.kesheshyan@gmail.com
 */

var ImgCrop = function(element, options) {
	var elem = $(element);
	var obj = this;
	

	var defaults = {
		frameWidth:  30,
		frameHeight: 30
	};

	var config = $.extend(defaults, options || {});
	
	var frameWidth  = config.frameWidth;
	var frameHeight = config.frameHeight;
	
	var _image       = elem.find(".img-holder img");
	var _imageHolder = elem.find(".img-holder");
	var _cropFrame   = elem.find(".crop-frame");
	var _boxFrame    = elem.find(".upload-box");
	
	//var _imageCropWidth;
	//var _imageCropHeight;
	var _imageMinWidth;
	var _imageMaxWidth;
	
	/**
	 * Init zoom slider
	 */
	var initSlide = function(container){
		var _slider = container.find(".scroll").slider({
			value: 50,
			step: 1,
			animate: false,
			slide: function( event, ui ) {
				if(_image.attr("src").indexOf("none.gif") == -1) {
					/*
					_zoomFactor = (ui.value / _boxFrame.width());
					_zoomDifference = _image.width() / (_imageCropWidth  * _zoomFactor);
					
					var _imgBeforeCenterPosition = {
						x: _imageHolder.width() / 2 - _image.position().left,
						y: _imageHolder.height() / 2 - _image.position().top
					}
					
					var _imgAfterCenterPosition = {
						x: _imgBeforeCenterPosition.x * _zoomDifference,
						y: _imgBeforeCenterPosition.y * _zoomDifference
					}
					
					var _offset = {
						w: -_imgAfterCenterPosition.x + _imgBeforeCenterPosition.x,
						h: -_imgAfterCenterPosition.y + _imgBeforeCenterPosition.y
					}
					*/
					_image.css({
						//"left" :   _image.position().left - _offset.w + "px",
						//"top" :    _image.position().top - _offset.h + "px",
						//"width":   _imageCropWidth  * _zoomFactor + "px",
						//"height":  _imageCropHeight * _zoomFactor + "px"
						"width":   _imageMinWidth + (_imageMaxWidth - _imageMinWidth) / 100 * ui.value  + "px"
					})
					resizeImageHolder();
				}
			}
		});
		_imageHolder.data("slider", _slider);
	}
	
	/**
	 * Recalculate image max and min
	 * zoom parametrs.
	 */
	var recalcMaxAndMinImgParams = function () {
		_imageMinWidth = _boxFrame.width() - config.frameWidth * 2;
		_imageMaxWidth = _boxFrame.width() + config.frameWidth * 4;
	}
	
	/**
	 * Getter for frame dimension
	 * @return object {w, h}
	 */
	this.getFrameDimension = function(){
		return {
			w: frameWidth,
			h: frameHeight
		}
	}
	
	/**
	 * Setter for frame dimension
	 * @param object {w, h}
	 */
	this.setFrameDimension = function(dimension){
		frameWidth  = dimension.w,
		frameHeight = dimension.h
		initFrame();
		resizeImageHolder();
	}
	
	/**
	 * Get curent crop params
	 * return Object {imgWidth, imgHeight, cropWidth, cropHeight, cropLeft, cropTop}
	 */
	this.getCropParams = function(){
		
		return {
			imgID:     _imageHolder.attr("id"),
			imgWidth:  _image.width(),
			imgHeight: _image.height(),
			cropWidth: _boxFrame.width() - frameWidth * 2,
			cropHeight: _boxFrame.height() - frameHeight * 2,
			cropLeft: _image.position().left + _imageHolder.position().left - frameWidth,
			cropTop: _image.position().top + _imageHolder.position().top - frameHeight
		}
	}
	
	
	/**
	 * set frame dimension
	 */
	var initFrame = function() {
		_width  = _boxFrame.width()  - frameWidth;
		_height = _boxFrame.height() - frameHeight;
		
		_left = _boxFrame.find(".crop-frame-left");
		_top = _boxFrame.find(".crop-frame-top");
		_right = _boxFrame.find(".crop-frame-right");
		_bottom = _boxFrame.find(".crop-frame-bottom");
		
		_boxFrame.find(".crop-frame-left div, .crop-frame-right div").css({"height": _height - frameHeight + "px"});
		_boxFrame.find(".crop-frame-top div, .crop-frame-bottom div").css({"width" : _width  - frameWidth + "px"});
		//set frame coords
		_left.css({
			"width": frameWidth + "px",
			"height" : _height + "px"
		});
		_right.css({
			"width": frameWidth + "px",
			"height" : _height + "px"
		});
		_top.css({
			"width" : _width + "px",
			"height": frameHeight + "px"
		});
		_bottom.css({
			"width" : _width + "px",
			"height": frameHeight + "px"
		});
	}
	
	/**
	 * Image center position
	 */
	var centerImagePos = function (){
		_offsetX = (_imageHolder.width()  - _image.width()) / 2;
		_offsetY = (_imageHolder.height() - _image.height()) / 2;
		_image.css({
			"left": _offsetX + "px",
			"top": _offsetY + "px"
		});
	}

	
	/**
	 * Resize image holder
	 */
	var resizeImageHolder = function (){
		
		var _boxFrameWidth  = _boxFrame.width ()  - (frameWidth * 2) ;
		var _boxFrameHeight = _boxFrame.height () - (frameHeight * 2);
		
		_imageHolderWidth  = ( _image.width ()*2 - _boxFrameWidth);
		_imageHolderHeight = ( _image.height()*2 - _boxFrameHeight);
		_imageHolderLeft   = ( -( _image.width () - _boxFrameWidth));
		_imageHolderTop    = ( -( _image.height() - _boxFrameHeight));
		
		_imageHolderWidth  = (_imageHolderWidth < _boxFrameWidth)? _boxFrameWidth : _imageHolderWidth;
		_imageHolderHeight = (_imageHolderHeight < _boxFrameHeight)? _boxFrameHeight : _imageHolderHeight;
		
		_imageHolderLeft = (_imageHolderWidth  <= _boxFrameWidth) ? frameWidth : _imageHolderLeft + frameWidth;
		_imageHolderTop  = (_imageHolderHeight <= _boxFrameHeight) ? frameHeight : _imageHolderTop + frameHeight;
		
		_imagePositionRight  = (_image.position().left + _image.width())  - _imageHolderWidth;
		_imagePositionBottom = (_image.position().top  + _image.height()) - _imageHolderHeight;
		
		if(_imagePositionRight >= 0) {
			_image.css({"left": _image.position().left - _imagePositionRight + "px"});
		}
		if(_imagePositionBottom >= 0) {
			_image.css({"top": _image.position().top - _imagePositionBottom + "px"});
		}
		
		_imageHolder.css({
			"width" : _imageHolderWidth  + "px",
			"height": _imageHolderHeight + "px",
			"left"  : _imageHolderLeft + "px",
			"top"   : _imageHolderTop  + "px"
		});
		
		
	}
	/**
	 * Refrashe image width and height
	 */
	this.refreshImage = function(){
		_image = _imageHolder.find("img");
		_image.width((_imageMinWidth + _imageMaxWidth) / 2);
		recalcMaxAndMinImgParams();
		_imageHolder.data("slider").slider( "value", 50 );
		resizeImageHolder();
		centerImagePos();
	}
	
	/**
	 * Main init function
	 */
	var init = function() {
		initSlide(elem);
		// set image dimensions
		recalcMaxAndMinImgParams();
		resizeImageHolder();
		initFrame();
		centerImagePos();
		// init drag
		_image.draggable({containment: "parent"});
		//_image.draggable();
	}
	
	init();
}	
$.fn.imgcrop = function(options) {
	return this.each(function() {
		var element = $(this);

		// Return early if this element already has a plugin instance
		if (element.data('imgcrop'))
			return;

		// pass options to plugin constructor
		var imgcrop = new ImgCrop(this, options);

		// Store plugin object in this element's data
		element.data('imgcrop', imgcrop);
	});
};


/**
 * Written testimonials drag plugin
 * @autor Ruslan Kesheshyan aka M.O.C.K. ruslan.kesheshyan@gmail.com
 */

var ImgDrag = function(element, options){
	var elem = $(element);
	var obj = this;
	var defaults = {};
	var config = $.extend(defaults, options || {});

	var _image       = elem.find(".img-drag-holder img");
	var _imageDragHolder = elem.find(".img-drag-holder");
	
	// init function
	var init = function() {
		var _imageHolderHeight = ( _image.height()*2  - elem.height());
		var _imageHolderTop    = ( -( _image.height() - elem.height()));
		_imageDragHolder.css({
			"position": "absolute",
			"left": 0,
			"top":  _imageHolderTop + "px",
			"width": "100%",
			"height": _imageHolderHeight + "px"
		})
		_image.css({"cursor": "move"});
		_image.draggable({containment: "parent"});
	}
	
	init();
}

$.fn.imgdrag = function(options) {
	return this.each(function() {
		var element = $(this);

		// Return early if this element already has a plugin instance
		//if (element.data('imgdrag'))
		//	return;
		// pass options to plugin constructor
		var imgcrop = new ImgDrag(this, options);

		// Store plugin object in this element's data
		element.data('imgdrag', imgcrop);
	});
};


