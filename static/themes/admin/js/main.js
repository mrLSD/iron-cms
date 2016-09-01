$(function(){
	//$(".s4").dropdownchecklist( { maxDropHeight: 250, width: 250 } );
	/*var promoPlayer = jwplayer("html5-video-promo").setup({
		file: 'http://www.youtube.com/watch?v=CxxPqayx8rU&list=UUGhV2hcAVLHRxFUdSaSvxkA',
		image: "<?=$this->btp?>images/promo.png",
		autostart : false,
		width: 600,
		height: 338
	});
	alert(1);
	jwplayer("html5-video-promo2").setup({
		file: 'http://www.youtube.com/watch?v=CxxPqayx8rU&list=UUGhV2hcAVLHRxFUdSaSvxkA',
		image: "<?=$this->btp?>images/promo.png",
		autostart : false,
		width: 600,
		height: 338
	});
	$(document.body).on("click", '.promovideo', function() {
			player.play();
			console.log(player);
			//jwplayer("html5-video-promo").setup({
			//		autostart : true
			//});
	});*/
// A $( document ).ready() block.

	/**
	 * Delete Item
	 */
	$(document.body).on("click", '.delete a', function() {
		var _element = this,
		    $li = $("#list_"+$(_element).attr("id")),
		    $ul = $li.parents('ul'),
		    $div = $li.parents('div');
		var _el_id = $div.attr("id");
		messageBoxDelete(function(action){
			if (action == "Ok") {
		    	$.post($(_element).attr("href"),{menuid:$(_element).attr("id")},function(data){
					$.fn.yiiListView.update( _el_id, {
							'complete': function(xhr,type)
							{
								// Update commot Sort List
								updateSortable( ".sort-list, .sortatable, .page-list" );
							}
						} );
			    });
			}
	    });
	    return false;
	});

    /**
     * Show/hide items
     */
	$(document.body).on("click", '.visible a, .hidde a', function() {
        // Get ID of changed element
        var $this = $(this);
        var $li = $this.parent('li');
        var $href = $this.attr('href');
        var $id = $li.attr("id");
        var $class = $li.attr('class');
        $id = $id.split("_")[1];
        if( $li.hasClass('visible') )
        {
			$li.removeClass('visible');
            $li.addClass('hidde');
			// Check is At Home button
			if( $li.hasClass("athome") )
				$this.text('Not at Home');
			if( $li.hasClass("free") )
				$this.text('Платное');
            $("#em_title_"+$id).show();
        } else
        {
			$li.removeClass('hidde');
            $li.addClass('visible');
			// Check is At Home button
			if( $li.hasClass("athome") )
				$this.text('Аt Home');
			if( $li.hasClass("free") )
				$this.text('Бесплатное');
            $("#em_title_"+$id).hide();
        }
        $.get( $href );
        return false;
    });

	/**
	 * Init Popup via Fancybox
	 */
	$(".popup-link").on("click", function() {
		var $this = $(this);
		$.fancybox({
			'speedIn'	 : 600,
			'speedOut'	 : 200,
			'overlayShow'	 : true,
			'showCloseButton': true,
			'href'		 : $this.attr("href")
		});
		return false;
	});

	/**
	 * Init sortable Nester Sortable for Procedures only
	 */
	$('ul.knowledgesortable').each(function(){
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

			start: function(event, ui) {
				$('.sortable').sortable("destroy");
			},

			stop: function(event, ui){
				updateSortable( ".sortable" );
				var hiered = $(this).nestedSortable('toArray', {startDepthCount: 0});
				var _result = {
					sectionID: $(this).attr("id"),
					sortOrder: hiered
				};
				$.post("/admin/pages/sort/", {
					'data': _result
				});
			}
		});
	});

	/**
	 * Global function for Sortable
	 * @author Evgeny Ukhanov
	 */
	function fnSortable(event, ui){
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
		};
		$.post("/admin/pages/sort/", {
			'data': _result
		});
	}

	/**
	 * Global function for Widgets Sortable
	 * @author Evgeny Ukhanov
	 */
	function fnSortableForWidgets(event, ui){
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
		};
		$.post("/admin/widgets/sort/", {
			'data': _result
		});
	}

	$(document.body).on("change",".filter-classes", function(){
		var _teachers = $("#teachers").val();
		var _styles = $("#styles").val();
		var _levels = $("#levels").val();
		var _durations = $("#durations").val();
		var _specificuse = $("#specificuse").val();
		var _series = $("#series").val();
        var _inhome = $("#inhome").val();
		$.post("/admin/video/filter", {
			teachers	: _teachers,
			styles		: _styles,
			levels		: _levels,
			durations	: _durations,
			specificuse	: _specificuse,
			series		: _series,
            inhome		: _inhome,
            bodies		: $("#bodies").val()
		}, function(content){
			$('#div_main').html(content);
			updateSortable( ".sortable" );
		});
		return false;
	});

	$(document.body).on('click', '.change-date', function(){
		$( ".datepicker" ).datepicker('show');
		return false;
	});

	if( $( ".datepicker" ).datepicker !== undefined ){
		$( ".datepicker" ).datepicker({
			dateFormat: "dd-mm-yy"
		});
	}


	/**
	 * Init sortable for specific selectors
	 * @author Evgeny Ukhanov
	 */
	function updateSortable( selector )
	{
		$( selector ).sortable({
			stop: fnSortable,
			cursor: 'move'
		});
	}

	/**
	 * Init sortable for Current selectors
	 * @author Evgeny Ukhanov
	 */
	//updateSortable(".sortable");

    $('#menu').metisMenu();
});

/**
 * Message Box Confirm for Delete
 * @param {function} callbackFunc - function for confirm handler
 */
function messageBoxDelete(callbackFunc){

	if( confirm("Вы уверены что хотите удалить это?") ) {
		callbackFunc('Ok');
	} else {
		callbackFunc('Cancel');
	}

	//AttentionBox.showMessage("Are you sure you want to delete this item?", {
	//	modal: true,
	//	buttons: [{
	//		caption: "Cancel",
	//		important: true
	//	}, {
	//		caption: "Ok"
	//	}],
	//	callback: callbackFunc
	//});
}

/**
 * Message Box Alert
 * @param {String} message
 */
function mAlert(message){
	alert(message);
	//AttentionBox.showMessage(message, {
	//	modal: true,
	//	buttons: [{
	//		caption: "Ok"
	//		}]
	//});
}

