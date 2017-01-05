$(function(){
	
	$(".sort-list, .sortatable").sortable({
		stop: function(event, ui){
			_items = $(this).children("li");
			_sortResultArray = [];
			_items.each(function(){
				_sortResultArray.push({
					parent_id: "root",
					item_id: $(this).attr("id").slice(5)
				});
			});
			_result = {
				sectionID: $(this).attr("id"),
				sortOrder: _sortResultArray
			}
			$.post("/admin/pages/sort/", {
				'data': _result
			});
		},
		cursor: 'move'
	});
	
	$('.box-content > ul.page-list').each(function(){
		var sortLevel = $(this).attr("data-sort-level") || 1;
		$(this).nestedSortable({
			forcePlaceholderSize: true,
			handle: 'div',
			helper:	'clone',
			items: 'li.sort-item',
			maxLevels: sortLevel,
			opacity: 0.6,
			placeholder: 'placeholder',
			revert: 250,
			tabSize: 25,
			tolerance: 'pointer',
			toleranceElement: '> div',
			listType: 'ul',
			
			stop: function(event, ui){
				var hiered = $(this).nestedSortable('toArray', {startDepthCount: 0});
				var _result = {
					sectionID: $(this).attr("id"),
					sortOrder: hiered
				}

				$.post("/admin/pages/sort/", {
					'data': _result
				});
			}
		});
	});
	

	
	
	/**
	 * Sort image at Gallery Set
	 * @version SaaS v5
	 */
	
	
	$(".gallery-set-list").sortable({
		stop: function(event, ui){
			_items = $(this).children("li");
			_sortResultArray = [];
			_items.each(function(){
				_sortResultArray.push($(this).attr("id"));
			});
			_result = {
				sectionID: $(this).attr("id"),
				sortOrder: _sortResultArray
			}
			$.post("/admin/gallery/sort/", {
				'data': _result
			});
			redrawOddItem($(this).parents("#content"));
		},
		cursor: 'move'
	});
	
	
	// init tabs
	$(".tabset").jtabs();
	// show-additional
	$(".show-additional").click(function(){
		$(this).toggleClass("open").parents("form").find(".additional-options").slideToggle();
		return false;
	});
	// init drop menu
	$(".main-nav a").click(function(){
		$(this).parent().find("ul").slideToggle();
		if ($(this).parent().find("ul").length) 
			return false;
	});
	
	// init img crop
	
	$(".crop-box").imgcrop({
		frameWidth: 30,
		frameHeight: 30
	});
	
	
	var _cropBoxes = $(".crop-box").data("imgcrop");
	
	// set crop frame parametrs
	$(".img-height-box .btn-minus").live("click", function(){
		_h = ((_cropBoxes.getFrameDimension().h + 10) <= 140) ? _cropBoxes.getFrameDimension().h + 10 : 140;
		resizeCropBoxFrame(_h);
		return false;
	});
	
	$(".img-height-box .btn-plus").live("click", function(){
		_h = ((_cropBoxes.getFrameDimension().h - 10) >= 30) ? _cropBoxes.getFrameDimension().h - 10 : 30;
		resizeCropBoxFrame(_h);
		return false;
	});
	
	$(".deleteNewPhoto").live("click", function(){
		var id = this.id.split('del');
		messageBoxDelete(function(action){
			if (action == "Ok") {
				id = id[0]
				var idEl = $("#" + id + "inp").val();
				$.post('/admin/gallery/deleteMedia/', {
					id: idEl
				});
				$("#btn" + id).show();
				$("#btn" + id + "img").attr("src", "/images/none.gif").fadeIn().bind("load", function(){
					// refresh img position 
					$(this).parents(".crop-box").data("imgcrop").refreshImage();
				});
				$("#" + id + "del").hide();
				$("#ac_fl" + id).show()
			}
		});
		return false;
	});
	
	$(".deleteNewMediaPhoto").live("click", function(){
		var id = this.id.split('del');
		id = id[0];
		messageBoxDelete(function(action){
			if (action == "Ok") {
				var idEl = $("#photo_id" + id).val();
				$.post("/admin/media/deleteMedia/", {
					id: idEl
				});
				var count_empty = 0;
				for (var i = 0; i < 3; i++) {
					if ($("#img" + i).attr("src") == "/images/none.gif") {
						$("#upl"+i).hide();
						count_empty++;
					}
				}
				$("#img" + id).attr("src", "/images/none.gif").fadeIn();
				if (count_empty == 2) 
					$("#ac_fl_photo" + id).html('<script>AC_FL_Review_init("simple",' + id + ')</script>');
				else 
					$("#ac_fl_photo" + id).html('<script>AC_FL_Review_init("notsimple",' + id + ')</script>');
				$("#ac_fl_photo" + id).show().find(".img-holder").hide();
				
				$("#" + id + "del").hide();
			}
		});
		return false;
	});
	
	$(".deleteNewMediaPhotoVideo").live("click", function(){
		var id = this.id.split('del');
		id = id[0];
		var $this = $(this);
		messageBoxDelete(function(action){
			if (action == "Ok") {
				var uploadersCount = $this.parents(".upload-columns-holder").find('div[id^="uploaded_image"]:visible').length;
				var idEl = $("#photo_id" + id).val();
				$.post("/admin/media/deleteMedia/", {
					id: idEl
				});
				$("#uploaded_image"+id).hide();
				$("#img" + id).attr("src", "/images/none.gif").fadeIn();
				
				// Hide previous uploader
				/*var _next = parseInt(id)+1;
				if(_next < 3)
				    $("#upl"+_next).hide();*/
				
				$("#upl0, #upl1, #upl2").hide();
				
				$("#upl"+id).show();
				
				
				if(uploadersCount == 1)
				//if(id == 0)
				{
				    $("#ac_fl_photo" + id).html('<script>AC_FL_Review_init("simple",' + id + ')</script>');
				    $("#video_upload").show();
				}
				else
				    $("#ac_fl_photo" + id).html('<script>AC_FL_Review_init("notsimple",' + id + ')</script>');
				
			}
		});
		return false;
	});
	
	// Check is it pair
	$("#add-new-pair").live("click", function(){
		if ($("img[src=\'/images/none.gif\']").length == 2) {
			mAlert("Please upload one more image!");
			return false;
		}
		saveCropChanges();
		return false;
	});
	
	// init review drag written testimonial
	$(".image-upload-area .img-holder img").bind("load", function(){
		$(this).parents(".img-holder").imgdrag();
	});	
	
	// Delete Video in Media
	$(".delete-media-video-in-photo").live("click",function() {	    	    
	    messageBoxDelete(function(action){
		if (action == "Ok") {
			var _id = $("#video_id").val();			
			$("#upl0").show();
			$("#video_upload").show();
			$("#uploaded_video").hide();	
			$(".delete-media-video-in-photo").hide();									
			$.post("/admin/media/deleteMedia/", {
				id: _id
			});
		}
	    });	    
	    return false;
	});
	
	$(".date-input").datepicker({
		showOn: "both",
		dateFormat: "yy-mm-dd",
		buttonImage: "/themes/admin/images/icon-calendar.png",
		buttonImageOnly: true
	});

	/**
	 * Delete itme from main list
	 * @version SaaS v5
	 */
	$(".delete a").live("click",function() {
		var _element = this,
		    $li = $("#list_"+$(_element).attr("id")),
		    $ul = $li.parents('ul'),
		    $div = $li.parents('div');
		messageBoxDelete(function(action){
			if ( action == "Ok" ) {
			    //$li.fadeOut();
				$.post( 
					$(_element).attr("href"),
					{
						menuid : $(_element).attr("id")
					},
					function( data ) {
	
					    $.fn.yiiListView.update( $div.attr("id") );
					    if( $("#additional-top-navigation").attr('id') != undefined )
					    {				
							$.fn.yiiListView.update( 'additional-top-navigation' );
							$.fn.yiiListView.update( 'additional-footer-navigation' );			    
					    }
					}
				);
			}
		});
		return false;
	});
	
	
	$(".page-tree-holder li a").live('click',function(){
		var selectedClassName = "selected";
		var $this = $(this);
		var $inputTitle      = $this.parents("form").find("#Links_title");
		var $inputPageID = $this.parents("form").find("#Links_seo_id");
		
		if(
				($inputTitle.val() == "")
				||
				($inputTitle.val() == $this.parents(".page-tree-holder").find('a[data-page-id="' + $inputPageID.val() + '"]').text())
		) {
			$inputTitle.val($this.text());
		}
		// add/remove style classes from all items in tree
		$this.parents(".page-tree-holder").find("a").removeClass(selectedClassName);
		$this.addClass(selectedClassName);
		// past selected data in hidden field
		$inputPageID.val($this.attr("data-page-id"));
		return false;
	});
	
	// init fancybox
	/*$(".popup-link").fancybox({
		'speedIn'	 : 600, 
		'speedOut'	 : 200, 
		'overlayShow'	 : true,
		'showCloseButton': true
	});*/
	$(".popup-link").live("click", function() {
		var $this = $(this);
		$.fancybox({
			'speedIn'	 : 600, 
			'speedOut'	 : 200, 
			'overlayShow'	 : true,
			'showCloseButton': true,
			'href'			: $this.attr("href"),
			'onComplete': function(){
				initPageTree();
			}
		});
		
		return false;
	});
	
	$(".button-popup-cancel").live("click", function(){
		$.fancybox.close();
		return false;
	});
	// redrawOddItem
	redrawOddItem($("#content"));
	initPageTree();
});

function initPageTree(){
	$(".page-tree-holder").treeview({
		animated: "fast"
	});
}
/**
 * Redraw odd item in sort ite
 */
function redrawOddItem($parent) {
	//$($parent).find(".sort-item").removeClass("odd").filter(":odd").addClass("odd");
}

/**
 * Get crop parametres and request to resizing and crop image
 * @version SaaS v5
 */
function saveCropChanges(){
	// Check is both image of pair uploaded
	if ($("img[src='/images/none.gif']").length == 1) {
		mAlert('Please upload one more image!');
		return false;
	}
	// Set position index and increment it
	var pos_index = parseInt($("#position_index").val()) + 1
	$("#position_index").val(pos_index);
	var imgPhotos = [];
	var imgIDs = [];
	// Get image and parematres from each image and send request
	$(".crop-box").each(function(){
		// Get crop parametres
		var _x = $(this).data("imgcrop");
		_params = _x.getCropParams(); 
		// Git image relative path and put to array
		var img = "/images/uploaded/" + $("#" + _params.imgID + "inp").val() + ".jpg";
		imgPhotos.push(img);
		imgIDs.push($("#" + _params.imgID + "inp").val());
		// Send non async request with parametres
		$.ajax({
			async: false,
			type: "POST",
			url: "/admin/gallery/resizeMedia/",
			data: {
				fileName: img,
				imgWidth: _params.imgWidth,
				cropWidth: _params.cropWidth,
				imgHeight: _params.imgHeight,
				cropHeight: _params.cropHeight,
				cropLeft: _params.cropLeft,
				cropTop: _params.cropTop
			},
			success: function(data, textStatus, jqXHR){				
			}
		});
	});
	// Set HTML new Set at Gallery for images downloaded above
	setHtmlNewSet(imgPhotos, imgIDs);
	return true;
}

/**
 * For Gallery Set  created new HTML element for including downloaded and croped images above
 * @version SaaS v5
 */
function setHtmlNewSet(imgPhotos, imgIDs){
	// Get content with pairs
	var html = $('#uploads').html();
	// Count of image
	var col = parseInt($('#holders_count').val());
	// Increment counter 
	var col1 = col + 1;
	var col2 = col1 + 1;
	// Set new value for counter
	$('#holders_count').val(col + 2);
	//============================================================    
	// New uploaders content with AC FL uploader initialozation
	var content = '<div class="upload-area" id="ua' + col1 + '"><section>' +
	'<div class="column1" id="clm1' +
	col1 +
	'"><div id="btn' +
	col1 +
	'file"></div>' +
	'<input type="hidden" id="' +
	col1 +
	'inp" /><div class="column-inner">' +
	'<div class="crop-box"><div class="upload-box"><div class="img-holder" id="' +
	col1 +
	'">' +
	'<img src="/images/none.gif" id="btn' +
	col1 +
	'img"/></div><span>Before Image ' +
	'<a href="#" class="deleteNewPhoto" id="' +
	col1 +
	'del" style="display: none">Delete' +
	'</a></span><div class="crop-frame-left"><div></div></div><div class="crop-frame-top">' +
	'<div></div></div><div class="crop-frame-right"><div></div></div>' +
	'<div class="crop-frame-bottom"><div></div></div>' +
	'<div id="ac_fl' +
	col1 +
	'"><script>AC_FL_init("' + col1 + '")</script></div>' +
	'</div><div class="scroll-area">' +
	'<span class="left">-</span><span class="right">+</span><div class="scroll"></div></div></div></div></div>' +
	'<div class="column2" id="clm2' +
	col2 +
	'"><div id="btn' +
	col2 +
	'file"></div>' +
	'<input type="hidden" id="' +
	col2 +
	'inp" /><div class="column-inner">' +
	'<div class="crop-box"><div class="upload-box"><div class="img-holder" id="' +
	col2 +
	'">' +
	'<img src="/images/none.gif" id="btn' +
	col2 +
	'img"/></div><span>After Image ' +
	'<a href="#" class="deleteNewPhoto" id="' +
	col2 +
	'del" style="display: none">Delete</a></span>' +
	'<div class="crop-frame-left"><div></div></div><div class="crop-frame-top"><div></div></div>' +
	'<div class="crop-frame-right"><div></div></div><div class="crop-frame-bottom"><div></div></div>' +
	'<div id="ac_fl' +
	col2 +
	'"><script>AC_FL_init("' + col2 + '")</script></div>' +
	'</div><div class="scroll-area">' +
	'<span class="left">-</span><span class="right">+</span><div class="scroll"></div></div></div></div></div>' +
	'<div class="column3" id="clm3' +
	col1 +
	'"><span class="add-new-pair-holder"><a href="#" id="add-new-pair" class="btn-4"><em class="add"></em><span>Add this pair</span></a></span>' +
	'<div class="img-height-box"><span>Image Height:</span>' +
	'<a href="/images/none.gif" class="btn-minus">-</a><a href="#" class="btn-plus">+</a></div></div></section>' +
	'</div>';
	//============================================================    
	// Erase +/- buttons
	$(".img-height-box").html('');
	// Content with already downloaded photos
	var addNewPhotoContent = '<li id="' + imgIDs[0] + '_' + imgIDs[1] + '">' +
	'<div class="upload-area" id="uploaded' +
	imgIDs[0] +
	'">' +
	'<section><div class="column1"><div class="set-photo-holder">' +
	'<img src="' +
	imgPhotos[0] +
	'" alt="" />' +
	'</div></div><div class="column2"><div class="set-photo-holder">' +
	'<img src="' +
	imgPhotos[1] +
	'" alt="" />' +
	'</div></div><div class="column3">' +
	'<a href="#" class="btn-9" onclick="deletePhotos(\'uploaded' +
	imgIDs[0] +
	'\',' +
	imgIDs[0] +
	',' +
	imgIDs[1] +
	');return false;"><em class="delete"></em><span>delete photos</span></a>' +
	'</div></section></div></li>';

	// Set contents
	$('#uploads').html(content);
	$('.gallery-set-list').append(addNewPhotoContent);
	
	// Reinit crop
	$(".crop-box").imgcrop({
		frameWidth: 30,
		frameHeight: 30
	});	
}

/**
 * Delete photos pair at Gallery Set 
 * @version SaaS v5
 */
function deletePhotos(element, id1, id2){
	messageBoxDelete(function(action){
		if (action == "Ok") {
			$("#" + element).hide();
			$.post("/admin/gallery/deleteMedia/", {
				id: id1
			});
			if (id2 != 0) 
				$.post("/admin/gallery/deleteMedia/", {
					id: id2
				});
		}
	});
}

function deleteMedia(element, id){
	messageBoxDelete(function(action){
		if (action == "Ok") {
			$("#" + element).hide();
			$.post("/admin/media/deleteMedia/", {
				id: id
			});
			$(".upload-area-spacer").removeClass("video-uploaded-area");
		}
	});
}

function deleteMediaVideo(id){
	messageBoxDelete(function(action){
		if (action == "Ok") {
			var _html = $("#upload_area_content").html();
			$(".upload-area").html(_html);
			$.post("/admin/media/deleteMedia/", {
				id: id
			});
		}
	});
}

function deleteMediaVideoInPhoto(id){
	messageBoxDelete(function(action){
		if (action == "Ok") {
			var _html = $("#upload_area_content2").html();
			$("#upload_video").html(_html);
			
			$("#upl0").show();
			$("#img0").attr("src", "/images/none.gif").fadeIn();
			$("#upload_video").show();
			$("#ac_fl_photo0").html('<script>AC_FL_Review_init("simple",0)</script>');
			$("#ac_fl_photo0").show();
			
			$("#0del").hide();
			
			$.post("/admin/media/deleteMedia/", {
				id: id
			});
		}
	});
}

function load_complete(id) 
{
	var result = id.split(",");	
	if (result[0] == 'success') {
		var current_element = result[4];		
		$("#ac_fl" + current_element).hide();		
		$("#btn" + current_element + "img").attr("src", result[3] + "?rand=" + Math.random() * new Date()).fadeOut().bind("load", function(){
			$(this).fadeIn();
			// refresh img position after load
			$(this).parents(".crop-box").data("imgcrop").refreshImage();
			$(this).parents('div[class^="column"]').find(".qq-uploader").fadeOut();
		});
		$("#" + current_element + "inp").val(result[1]);
		$("#" + current_element + "del").show();
	}
	else 
		if (result[0] == 'successLetter') {
			$(".img-holder img").attr("src", '/images/uploaded/' + result[1] + '.jpg?rand=' + Math.random() * new Date()).bind("load", function(){
				$(this).parents(".img-holder").imgdrag();
			});
		}
		else 
		if (result[0] == 'successMedia') {		    
			$("#video_upload").hide();
			var id = result[2];
			var next = parseInt(id) + 1;
			// Hide current uploader
			$("#upl" + id).hide();
			// Show next uploader
			if (next < 3) 
				$("#upl" + next).show();
			// Show uploaded Image
			$("#img" + id)
				.attr('src', '/images/uploaded/' + result[1] + '.jpg')
				.parent().attr("href", '/images/uploaded/' + result[1] + '.jpg');
			$("#uploaded_image" + id).show();						
			$("#photo_id" + id).val(result[1]);									
		}
		else 
		if (result[0] == 'successVideophoto') {
			$("#ac_fl_photo").hide();
			$("#img1").attr('src', '/images/uploaded/' + result[1] + '.jpg');
			$("#del1").show();
			$("#photo_id").val(result[1]);
		}
	if (result[0] == 'successCommon') {	    
	    $(".img-holder img").attr("src", result[2] + '?rand=' + Math.random() * new Date()).bind("load", function(){
		$(this).parents(".img-holder").imgdrag();
	    });
	    //$("#media_id").val(result[1]);
	    AC_FL_CommonUploaded(250,400,0);
	} 	    
}

/**
 *  Flash uploader init for Gallery
 *  @param {String} element - DOM element
 */
function AC_FL_init(element){
	var id = $("#model_id").val() + ',' + $("#position_index").val() + ',' + element;
	var upload_script = '/admin/gallery/uploadFL/';
	var file_name = 'snapshot';
	var dir = '';
	var quality = 100;
	var max_width = 400;
	
	var HTML_width = '225';
	var HTML_height = '261';
	var component_url = '/components/ac_fl_uploader/uploader';
	var component_url_swf = '/components/ac_fl_uploader/uploader.swf';

	AC_FL_html = '<object type="application/x-shockwave-flash" data="'+component_url_swf+
	    '" width="' + HTML_width + '" height="' + HTML_height + 
	    '">'+
	    '<param name="movie" value="' +
	    component_url_swf +
	    '" />' +
	    '<param name="quality" value="high" />' +
	    '<param value="transparent" name="wmode" />' +
	    '<param name="flashvars" value="id=' +
	    id +
	    '&upload_script=' +
	    upload_script +
	    '&dir=' +
	    dir +
	    '&file_name=' +
	    file_name +
	    '&q=' +
	    quality +
	    '&max_width=' +
	    max_width +
	    '" />'+
	    '</object>';
	    
	$("#ac_fl" + element).html(AC_FL_html);
}

/**
 * Initialize FLASH-photo  uploader for Review Written Testimonials, when we uploadin photo
 * @version SaaS v5
 * @version SaaS v5
 */
function AC_FL_Letters_init(){
	var id = $("#model_id").val();
	var upload_script = '/admin/review/uploadFLLetters/';
	var file_name = 'snapshot';
	var dir = '';
	var quality = 100;
	var max_width = 650;
	
	var HTML_width = '129';
	var HTML_height = '25';
	var component_url = '/components/ac_fl_uploader/uploader';
	var component_url_swf = '/components/ac_fl_uploader/uploader_letters.swf';
	
	AC_FL_html = '<object classid="clsid:D27CDB6E-AE6D-11cf-96B8-444553540000" width="' + HTML_width + '" height="' + HTML_height + '" title="">' +
	'<param name="movie" value="' +
	component_url_swf +
	'">' +
	'<param name="quality" value="high">' +
	'<embed quality="high" pluginspage="http://www.adobe.com/shockwave/download/download.cgi?P1_Prod_Version=ShockwaveFlash" ' +
	'type="application/x-shockwave-flash" width="' +
	HTML_width +
	'" height="' +
	HTML_height +
	'" ' +
	'src="' +
	component_url_swf +
	'" menu="false" ' +
	'flashvars="id=' +
	id +
	'&upload_script=' +
	upload_script +
	'&dir=' +
	dir +
	'&file_name=' +
	file_name +
	'&q=' +
	quality +
	'&max_width=' +
	max_width +
	'">' +
	'</embed>' +
	'</object>';
	$("#ac_fl").html(AC_FL_html);
}

/**
 * Initialize FLASH-photo  uploader for Media section #2 and #3, when we uploadin photo
 * In this type we are have two  types of FLASH
 */
function AC_FL_Review_init(type, count){
	var id = $("#model_id").val() + ',' + count;
	var upload_script = '/admin/media/uploadFLPhoto/';
	var file_name = 'snapshot';
	var dir = '';
	var quality = 100;
	var max_width = 650;
	
	if (type == "simple") {
		var HTML_width = '283';
		var HTML_height = '239';
	}
	else {
		var HTML_width = '179';
		var HTML_height = '259';
	}
	var component_url = '/components/ac_fl_uploader/uploader';
	if (type == 'simple') 
		var component_url_swf = '/components/ac_fl_uploader/uploader_video.swf';
	else 
		var component_url_swf = '/components/ac_fl_uploader/uploader_photo.swf';

	AC_FL_html = '<object type="application/x-shockwave-flash" data="'+component_url_swf+
	    '" width="' + HTML_width + '" height="' + HTML_height + 
	    '">'+
	    '<param name="movie" value="' +
	    component_url_swf +
	    '" />' +
	    '<param name="quality" value="high" />' +
	    '<param value="transparent" name="wmode" />' +
	    '<param name="flashvars" value="id=' +
	    id +
	    '&upload_script=' +
	    upload_script +
	    '&dir=' +
	    dir +
	    '&file_name=' +
	    file_name +
	    '&q=' +
	    quality +
	    '&max_width=' +
	    max_width +
	    '" />'+
	    '</object>';
	    
	$("#ac_fl_photo" + count).html(AC_FL_html);
}

/**
 * Initialize FLASH-photo  uploader for Media section #3, when we uploadin photo
 */
function AC_FL_VideoPhoto_init(){
	var id = $("#model_id").val() + ',';
	var upload_script = '/admin/media/uploadFLPhotoVideo/';
	var file_name = 'snapshot';
	var dir = '';
	var quality = 100;
	var max_width = 650;
	
	var HTML_width = '225';
	var HTML_height = '261';
	var component_url = '/components/ac_fl_uploader/uploader';
	var component_url_swf = '/components/ac_fl_uploader/uploader.swf';
	
	AC_FL_html = '<object classid="clsid:D27CDB6E-AE6D-11cf-96B8-444553540000" width="' + HTML_width + '" height="' + HTML_height + '" title="">' +
	'<param name="movie" value="' +
	component_url_swf +
	'">' +
	'<param name="quality" value="high">' +
	'<embed wmode="transparent" quality="high" pluginspage="http://www.adobe.com/shockwave/download/download.cgi?P1_Prod_Version=ShockwaveFlash" ' +
	'type="application/x-shockwave-flash" width="' +
	HTML_width +
	'" height="' +
	HTML_height +
	'" ' +
	'src="' +
	component_url_swf +
	'" menu="false" ' +
	'flashvars="id=' +
	id +
	'&upload_script=' +
	upload_script +
	'&dir=' +
	dir +
	'&file_name=' +
	file_name +
	'&q=' +
	quality +
	'&max_width=' +
	max_width +
	'">' +
	'</embed>' +
	'</object>';
	$("#ac_fl_photo").html(AC_FL_html);
}

/**
 * Init Tiny MCE tool
 */
function tinyMCE_init(){    
	tinyMCE.init({ 
		mode: "specific_textareas",
		editor_deselector : "small-desc-area",

		theme: "advanced",
		
		skin: "cirkuit",
		
		plugins: "safari,pagebreak,advlink,xhtmlxtras,inlinepopups,style,imagemanager,filemanager,contextmenu,paste,preview,advhr",
		//plugins: "safari,pagebreak,advlink,xhtmlxtras,inlinepopups,style,imagemanager,filemanager,contextmenu,paste,preview,pagebreak",
		//plugins: "safari,pagebreak,advlink,xhtmlxtras,inlinepopups,style,imagemanager,contextmenu,paste,preview,pagebreak",
		//plugins : "imagemanager,filemanager,pagebreak,style,layer,table,save,advhr,advimage,advlink,emotions,iespell,inlinepopups,insertdatetime,preview,media,searchreplace,print,contextmenu,paste,directionality,fullscreen,noneditable,visualchars,nonbreaking,xhtmlxtras,template,wordcount,advlist,autosave",
		//plugins: "safari,pagebreak,advlink,xhtmlxtras,inlinepopups,style,,,imagemanager,filemanager",
		
		//theme_advanced_buttons1: "formatselect,fontselect,fontsizeselect,|,bold,italic,underline,strikethrough,|,justifyleft,justifycenter,justifyright,justifyfull,|,bullist,numlist,|,link,unlink,anchor,|,pagebreak,preview,|,code,|,insertimage",
		theme_advanced_buttons1: "formatselect,fontsizeselect,|,bold,italic,underline,strikethrough,|,justifyleft,justifycenter,justifyright,justifyfull,|,outdent,indent,|,bullist,numlist,|,link,unlink,anchor,|,hr,preview,|,,code,|,insertimage,insertfile",
		theme_advanced_buttons2: "",
		
		// PASTE plagin options
		paste_auto_cleanup_on_paste : true,
		paste_strip_class_attributes : "all",
		paste_remove_styles : true,
		paste_remove_spans : true,
		
		// Pagebreaker
		pagebreak_separator : "<!-- page_break -->",
		
		// Preview
		plugin_preview_width : "612",
		plugin_preview_height : "600",		
		
		theme_advanced_toolbar_location: "top",
		theme_advanced_toolbar_align: "left",
		theme_advanced_statusbar_location: "bottom",
		theme_advanced_resizing: true
	});
	
	tinyMCE.init({ 
		mode: "specific_textareas",
		editor_selector : "small-desc-area",
		theme: "advanced",
		
		skin: "cirkuit",
		
		plugins: "safari,pagebreak,advlink,xhtmlxtras,inlinepopups,style,imagemanager,filemanager,contextmenu,paste,preview,advhr",
		theme_advanced_buttons1: "bold,italic,underline",
		theme_advanced_buttons2: "",
		
		// PASTE plagin options
		paste_auto_cleanup_on_paste : true,
		paste_strip_class_attributes : "all",
		paste_remove_styles : true,
		paste_remove_spans : true,
		
		// Pagebreaker
		pagebreak_separator : "<!-- page_break -->",

		// Preview
		plugin_preview_width : "612",
		plugin_preview_height : "200",		
		
		theme_advanced_toolbar_location: "top",
		theme_advanced_toolbar_align: "left",
		theme_advanced_statusbar_location: "bottom",
		theme_advanced_resizing: true
	});
}

/**
 * Ajax upload
 * @param {String} id
 */
function ajaxUpload(id){
	var uploader = new qq.FileUploader({
		// pass the dom node (ex. $(selector)[0] for jQuery users)
		//element: document.getElementById("btn"+id+"file"),    
		element: $("#btn" + id + "file")[0],
		// path to server-side upload script
		// url of the server-side upload script, should be on the same domain
		action: "/admin/gallery/upload/",
		// additional data to send, name-value pairs
		params: {
			"element": id,
			"id": $("#model_id").val(),
			"position_index": $("#position_index").val()
		},
		// validation    
		// ex. ["jpg", "jpeg"
		allowedExtensions: [],
		// each file size limit in bytes
		// this option isn"t supported in all browsers
		sizeLimit: 0, // max size   
		minSizeLimit: 0, // min size
		// set to true to output server response to console
		debug: false,
		// events         
		// you can return false to abort submit
		onSubmit: function(id, fileName){
			$("#btn" + this.params.element).hide();
			//$("#btn" + this.params.element + "file").hide();
		},
		onProgress: function(id, fileName, loaded, total){
		},
		onComplete: function(id, fileName, responseJSON){
			if ($("#position_index").val() == "tst") 
				$("#position_index").val(responseJSON.position_index + "+")
			else 
				$("#position_index").val("0")
			$("#btn" + this.params.element + "img").attr("src", responseJSON.filename).fadeOut().bind("load", function(){
				$(this).fadeIn();
				// refresh img position after load
				$(this).parents(".crop-box").data("imgcrop").refreshImage();
				$(this).parents('div[class^="column"]').find(".qq-uploader").fadeOut();
			});
			$("#" + this.params.element + "inp").val(responseJSON.new_id);
			$("#" + this.params.element + "del").show();
		},
		onCancel: function(id, fileName){
		},
		messages: {},
		showMessage: function(message){
			mAlert(message);
		}
	});
}

/**
 * Resize crop frame
 * @param {Object} _h  - frame height
 */
function resizeCropBoxFrame(_h){
	$(".crop-box").each(function(){
		var _x = $(this).data("imgcrop");
		_x.setFrameDimension({
			w: 30,
			h: _h
		});
	});
}

/**
 * For Media/Review video upload
 * Init TransLoadIt API tool and set hanlder for it
 * @param {String} module_id - URL path element
 * @param {String} model_id - сurrent id of model (item)
 */
function initUploadVideoAndForm(module_id, model_id){
	$("#upload_video").live("change", function(){
		$("#upload-form").submit();
	});
	
	$("#upload-form").transloadit({
		modal: true,
		wait: true,
		autoSubmit: false,
		onSuccess: function(assembly){
			$(".upload-area").html('<section id="sectionVideo"><div id="flashcontent"><img src="' + assembly.results.thumbs[0].url + '"></div></section>');
			$(".upload-area-spacer").addClass("video-uploaded-area");
			$.post("/" + module_id + "/media/uploadexternal/", {
				assembly: assembly,
				id: model_id
			}, function(data){
				$("#sectionVideo").append('<a href="#" class="btn-9" onclick="deleteMediaVideo(\'' + data + '\');return false;"><em class="delete"></em><span>delete</span></a>');
				$note.show("Video successeful uploaded", "confirm", {
					"width": 250,
					"lifetime": 3000,
					"autoremove": true,
					"anchor": "tl"
				});
			});
		}
	});
}

/**
 * Video uploader for TransLoadIt tool for Media
 * @version SaaS v5
 */
function initUploadVideoPhoto(module_id, model_id) {
	// Set submit handler for TransLoadIt tool
	$("#upload_video_for_VideoPhoto").live("change", function(){
		$("#upload-form2").submit();
	});
	
	// TransLoadIt API tool
	$("#upload-form2").transloadit({
		modal: true,
		wait: true,
		autoSubmit: false,
		onStart: function(assembly){
			for(var i=0; i<3; i++)
			    $("#upl"+i).hide();
		},
		onSuccess: function(assembly){
			$("#flashcontent").html('<img src="' + assembly.results.thumbs[0].url + '">');
			$("#video_upload").hide();
			$("#uploaded_video").show();			

			$.post("/" + module_id + "/media/uploadexternal/", {
				assembly: assembly,
				id: model_id
			}, function(data){
				// If success - show DELETE button
				$("#video_id").val(data);
				$(".delete-media-video-in-photo").show();				
				//$("#sectionVideo").append('<a href="#" class="btn-9" onclick="deleteMediaVideoInPhoto(\'' + data + '\');return false;"><em class="delete"></em><span>delete</span></a>');
				// Show Notification about success
				$note.show("Video successeful uploaded", "confirm", {
					"width": 250,
					"lifetime": 3000,
					"autoremove": true,
					"anchor": "tl"
				});
			});
		}
	});	
}
/**
 * Init TransLoadIt API tool and set hanlder for it
 * This tool for section #3 in uploading video in Media
 * @param {String} module_id - URL path element
 * $param {String} model_id - ID of current item, where uploading video
 */
function initUploadVideoPhotoAndForm(module_id, model_id){
	// Set submit handler for TransLoadIt tool
	$("#upload_video2").live("change", function(){
		$("#upload-form2").submit();
	});
	
	// TransLoadIt API tool
	$("#upload-form2").transloadit({
		modal: true,
		wait: true,
		autoSubmit: false,
		onStart: function(assembly){
			// When starting upload video - hide Image Upload elements
			for (var i = 0; i < 3; i++) {
				$("#upl" + i).hide();
			}
		},
		onSuccess: function(assembly){
			$("#upload_video").html('<section id="sectionVideo"><div id="flashcontent"><img src="' + assembly.results.thumbs[0].url + '"></div></section>');
			$(".upload-area-spacer").addClass("video-uploaded-area");
			$.post("/" + module_id + "/media/uploadexternal/", {
				assembly: assembly,
				id: model_id
			}, function(data){
				// If success - show DELETE button
				$("#sectionVideo").append('<a href="#" class="btn-9" onclick="deleteMediaVideoInPhoto(\'' + data + '\');return false;"><em class="delete"></em><span>delete</span></a>');
				// Show Notification about success
				$note.show("Video successeful uploaded", "confirm", {
					"width": 250,
					"lifetime": 3000,
					"autoremove": true,
					"anchor": "tl"
				});
			});
		}
	});
}

/**
 * Initialize/reinitialize Flash uploader for Written letters
 */
function newUploadLetter(){
	AC_FL_Letters_init();
	return false;
}

/**
 * For Written letters - Previous item
 */
function setPrevious(){
	$("#next").val('p');
	$("#categories-form").submit();
}

/**
 * For Written letters - Next item
 */
function setNext(){
	$("#next").val('n');
	$("#categories-form").submit();
}

/**
 * Video Player init
 * @param {String} video,
 * @param {String} image
 * @param {String} title
 */
function mediaVideoPlayerInit(video, image, title){
	var so = new SWFObject("/components/videoplayer/videoplayer.swf", "mymovie", "540", "380", "9", "#333333");
	so.addParam("menu", "false");
	so.addParam("allowfullscreen", "true");
	so.addParam("allowScriptAccess", "always");
	so.addVariable("setting", "9");
	so.addVariable("playerAutoHide", "yes");
	
	so.addVariable("videoWidth", "540");
	so.addVariable("videoHeight", "380");
	
	so.addVariable("imagePath", image);
	so.addVariable("videoPath", video);
	so.addVariable("videoDefaultQuality", "small");
	
	so.addVariable("playerNavigations", "ply,sek,vol,ful,txt");
	
	so.addVariable("videoAutoStart", "no");
	so.addVariable("reflection", "no");
	
	so.addVariable("titleVerticalSpace", "75");
	so.addVariable("videoTitle", "<b>" + title + "</b>");
	so.addParam("wmode", "transparent");
	so.addVariable("videoDescription", "This website is powered by <b>MediRocket</b> - Marketing solutions for Medical Industry. <b><a href='http://medirocket.com'>www.medirocket.com</a></b>");
	so.write("flashcontent");
}

/**
 * Message Box Confirm for Delete
 * @param {function} callbackFunc - function for confirm handler
 */
function messageBoxDelete(callbackFunc){
	AttentionBox.showMessage("Are you sure you want to delete this item?", {
		modal: true,
		buttons: [{
			caption: "Cancel",
			important: true
		}, {
			caption: "Ok"
		}],
		callback: callbackFunc
	});
}

/**
 * Message Box Alert
 * @param {String} message
 */
function mAlert(message){
	AttentionBox.showMessage(message, {
		modal: true,
		buttons: [{
			caption: "Ok"
		}]
	});
}

/**
 * Message Box Confirm for deleting URL based (href)
 * @param {Object} element - DOM element
 */
function deleteByLink(element){
	messageBoxDelete(function(action){
		if (action == "Ok") {
			window.location.href = $(element).attr("href");
		}
	});
}

/**
 * AC FL image uploader for Blog
 * @version SaaS v5
 */
function AC_FL_Common(max_width,max_height,crop)
{
	var id = $("#model_id").val();	
	var upload_script = '/admin/blog/uploadFL/';
	var file_name = 'snapshot';
	var dir = '';
	var quality = 100;
	//var max_width = 400;
	//var max_height = 50;
	//var crop = 1;
	
	var HTML_width = '283';
	var HTML_height = '239';
	var component_url = '/components/ac_fl_uploader/uploader';
	var component_url_swf = '/components/ac_fl_uploader/uploader_video.swf';		
	
	AC_FL_html = '<object classid="clsid:D27CDB6E-AE6D-11cf-96B8-444553540000" width="' + HTML_width + '" height="' + HTML_height + '" title="">' +
	    '<param name="movie" value="' + component_url_swf + '">' +
	    '<param name="quality" value="high">' +
	    '<embed quality="high" pluginspage="http://www.adobe.com/shockwave/download/download.cgi?P1_Prod_Version=ShockwaveFlash" ' +
	    'type="application/x-shockwave-flash" ' +
	    'width="'+ HTML_width + '" '+
	    'height="' + HTML_height + '" '+
	    'src="' + component_url_swf + '" '+
	    'menu="false" ' +
		'flashvars="id=' + id +
		'&upload_script=' + upload_script +
		'&dir=' + dir +
		'&file_name=' + file_name +
		'&q=' + quality +
		'&max_height='+ max_height +
		'&crop=' + crop +
		'&max_width=' + max_width + '">' +
	    '</embed>' +
	    '</object>';
	$("#ac_fl").html(AC_FL_html);   
}

/**
 * AC FL image uploader for Blog
 * @version SaaS v5
 */
function AC_FL_CommonUploaded(max_width,max_height,crop)
{
	var id = $("#media_id").val();
	var upload_script = '/admin/blog/uploadFL/';
	var file_name = 'snapshot';
	var dir = '';
	var quality = 100;
	//var max_width = 400;
	//var max_height = 150;
	//var crop = 1;	
	
	var HTML_width = '129';
	var HTML_height = '25';	
	var component_url_swf = '/components/ac_fl_uploader/uploader_blog.swf';
	
	AC_FL_html = '<object classid="clsid:D27CDB6E-AE6D-11cf-96B8-444553540000" width="' + HTML_width + '" height="' + HTML_height + '" title="">' +
	    '<param name="movie" value="' + component_url_swf + '">' +
	    '<param name="quality" value="high">' +
	    '<embed quality="high" pluginspage="http://www.adobe.com/shockwave/download/download.cgi?P1_Prod_Version=ShockwaveFlash" ' +
	    'type="application/x-shockwave-flash" ' +
	    'width="'+ HTML_width + '" '+
	    'height="' + HTML_height + '" '+
	    'src="' + component_url_swf + '" '+
	    'menu="false" ' +
		'flashvars="id=' + id +
		'&upload_script=' + upload_script +
		'&dir=' + dir +
		'&file_name=' + file_name +
		'&q=' + quality +
		'&max_height='+ max_height +	
		'&crop=' + crop +
		'&max_width=' + max_width + '">' +
	    '</embed>' +
	    '</object>';
	$("#ac_fl").html(AC_FL_html);   
}

/**
 * Changes of Page Type for Options button
 * @author Evgeny Ukhanov
 */
function changesOptions(element) {
    // If it Other Page type
    if(element == "other_page") {
	// If selected YES
	if($("#Menus_other_page").val() == 1){
	    $("#Menus_view_main").parent().parent().hide();
	    $("#Menus_home_page").parent().parent().hide();
	    $("#Menus_contact_page").parent().parent().hide();
	    $(".tabset").hide();	
	    $(".wysiwyg-area").hide();	
	} else { // Selected NO
	    $("#Menus_view_main").parent().parent().show();
	    $("#Menus_home_page").parent().parent().show();
	    $("#Menus_contact_page").parent().parent().show();
	    $(".tabset").show();	
	    $(".wysiwyg-area").show();		
	}
    }
    // If it Home Page type
    if(element == "home_page") {
	// If selected YES
	if($("#Menus_home_page").val() == 1) {
	    $("#Menus_view_main").parent().parent().show();
	    $("#Menus_other_page").parent().parent().hide();
	    $("#Menus_contact_page").parent().parent().hide();
	    $(".tabset").hide();	
	    $(".wysiwyg-area").hide();	
	} else { // Selected NO
	    $("#Menus_view_main").parent().parent().show();
	    $("#Menus_other_page").parent().parent().show();
	    $("#Menus_contact_page").parent().parent().show();	
	    $(".tabset").show();
	    $(".wysiwyg-area").show();
	}
    }
    // If it Contact Page type
    if(element == "contact_page") {
	// If selected YES
	if($("#Menus_contact_page").val() == 1) {
	    $("#Menus_view_main").parent().parent().show();
	    $("#Menus_other_page").parent().parent().hide();
	    $("#Menus_home_page").parent().parent().hide();
	    $(".tabset").show();	
	    $(".wysiwyg-area").show();	
	} else {  // Selected NO
	    $("#Menus_view_main").parent().parent().show();
	    $("#Menus_other_page").parent().parent().show();
	    $("#Menus_home_page").parent().parent().show();
	    $(".tabset").show();	
	    $(".wysiwyg-area").show();		
	}
    }
}