<?php include("stat.php");?>
	<!-- <script type="text/javascript" src="http://ajax.googleapis.com/ajax/libs/jquery/1.6.2/jquery.min.js"></script> -->
	<script type="text/javascript" src="http://www.google.com/jsapi"></script>
	<script type="text/javascript" src="/components/gastats/js/jquery.flot.js"></script>
	<script type="text/javascript" src="/components/gastats/js/jquery.graphtable-0.2.js"></script>
	<style type="text/css">
		.wrapper{
			width:100%;
			margin:0 auto;
			overflow:hidden;
		}
		.pie-column-holder{
			width:100%;
			overflow:hidden;
			padding:50px 0 0;
			margin-bottom:-20px;
		}
		.pie-column-holder .pie-chart-holder{
			float:left;
			width:450px;
		}
		.wrapper h3{
			font-size:18px;
			margin:0 0 10px;
		}
		.wrapper h2{
			font-size:24px;
			line-height:50px;
			padding:0 15px;
			margin:0 0 15px;
			background:#efefef;
			border-radius: 10px;
			-moz-border-radius: 10px;
			-webkit-border-radius: 10px;
			-o-border-radius: 10px;
			position:relative;
		}
		.pie-column-holder .total-values{
			overflow:hidden;
			background:#f1f9fc;
			padding:30px 50px;
			border-radius: 10px;
			-moz-border-radius: 10px;
			-webkit-border-radius: 10px;
			-o-border-radius: 10px;
		}
		.pie-column-holder .total-values ul{
			margin:0;
			padding:0;
			list-style:none;
			overflow:hidden;
		}
		.pie-column-holder .total-values li{
			float:left;
			width: 49%;
			font-weight:bold;
			font-size:18px;
			line-height:38px;
		}
		.pie-column-holder .total-values li em{
			color:#00366f;
			font-style:normal;
			font-size:15px;
		}
		/* table-column-holder */
		.table-column-holder{
			width:100%;
			overflow:hidden;
		}
		.table-column-holder #table_div{
			float:left;
			width:49%;
		}
		.table-column-holder #table_div table{width:100% !important;}
		.table-column-holder #table_div_key{
			float:right;
			width:49%;
		}
		.table-column-holder #table_div_key table{width:100% !important;}
		/* google-visualization-table-td */
		.google-visualization-table-td{
			text-align:right;
		}
		.google-visualization-table-td:first-child {
			text-align:left;
			font-weight:bold;
			color:#00366f;
		}
		.google-visualization-table-table td{
			padding:5px 10px;
			border-width:0 0 1px;
		}
		.google-visualization-table-th{background:#e6e6e6 !important;}
		
		/* flot */
		.space {
			display : block;
			height : 19px;
		}
		
		.flot-graph {
			display : inline-block;
			padding : 0;
			line-height : normal;
		}
		
		.flot-graph .xAxis {
			line-height : 7px;
		}
		
		.caption {
			float : left;
			margin : -3px 0 0 0;
			font-size : 12px;
			line-height : normal;
			color : #1c1c1c;
			font-weight : bold;
		}
		
		.flot-graph .legend {
			line-height : 12px;
		}
		
		.flot-graph .legend table {
			border : 0;
			width : auto;
		}
		
		.flot-graph .legend table td {
			border : 0;
			padding : 2px 0 0 5px;
			line-height : 18px;
			white-space : nowrap;
			width : auto;
		}
		
		#tooltip {
			background-color : #1c1c1c;
			padding : 0 8px;
			color : #FFFFFF;
			font-size : 11px;
			line-height : 25px;
			height: 25px;
		}
	</style>
	<script type="text/javascript">	  
		//google.load("visualization", "1", {packages:["piechart"]});
		google.setOnLoadCallback(drawPieChart);
		
		function drawPieChart() {
			var data = new google.visualization.DataTable();
			data.addColumn('string', 'Source');
			data.addColumn('number', 'Visits');
			data.addColumn('string', '% visits');
			data.addRows([
			              <?php //echo $monthCountrys?>
						   <?php echo $monthSource;?>
			            ]);
			
			var chart = new google.visualization.PieChart(document.getElementById('piechart_div'));
			chart.draw(data, {
				width: 400,
				height: 300,
				colors: [
				'#00366f',
				'#aa8300',
				'#aa3900',
				'#002348',
				'#00366f',
				'#090077',
				'#aa8300',
				'#aa6600',
				'#ff0000',
				'#ff4040',
				'#ff7373',
				'#a60000',
				'#ddbe25',
				'#9d9d9d',
				'#666793',
				'#6f4d0d',
				'#6db5b4',
				'#c41e82',
				'#c23931',
				'#281c02'
				],
				chartArea:{left:0,top:0,width:"100%",height:"75%"}
			});
		}
		google.load("visualization", "1", {packages:["table"]});
		google.setOnLoadCallback(drawSourceTable);
		google.setOnLoadCallback(drawKeywordTable);
		
		function drawSourceTable() {
			var data = new google.visualization.DataTable();
			data.addColumn('string', 'Sources');
			data.addColumn('number', 'Visits');
			data.addColumn('string', '% visits');
			data.addRows([
			              <?php echo $monthSource;?>
			            ]);
			
			var chart = new google.visualization.Table(document.getElementById('table_div'));
			chart.draw(data, {
				
				title: 'Sources'
			});
		}

		function drawKeywordTable() {
			var data = new google.visualization.DataTable();
			data.addColumn('string', 'Keyword');
			data.addColumn('number', 'Visits');
			data.addColumn('string', '% visits');
			data.addRows([
			              <?php echo $monthKeyword;?>
			            ]);
			
			var chart = new google.visualization.Table(document.getElementById('table_div_key'));
			chart.draw(data, {
				title: 'Keywords'
			});
		}
		//	drow main visits chart
		google.load("visualization", "1", {packages:["areachart"]});
		//google.setOnLoadCallback(drawBarChart);
		
		function drawBarChart() {
			var data = new google.visualization.DataTable();
			data.addColumn('string', 'Date');
			data.addColumn('number', 'Visits');
			data.addRows([
			              <?php echo $monthVisits?>
			            ]);
			
			barsVisualization = new google.visualization.AreaChart(document.getElementById('barchart_div'));
			barsVisualization.draw(data, {
				areaOpacity:0.1,
				height: 200,
				pointSize: 7,
				lineWidth: 2,
				colors: ['#28b3ff'],
				fontSize: 11,
				hAxis:{
					slantedText:false,
					showTextEvery:2
				},
				vAxis:{
					textPosition:'out'
				},
				chartArea:{left:"5%", width:"95%",height:"75%"}
				
			});
			// Add our over/out handlers.
			google.visualization.events.addListener(barsVisualization, 'onmouseover', barMouseOver);
			google.visualization.events.addListener(barsVisualization, 'onmouseout', barMouseOut);
		}
		
		function barMouseOver(e) {
			barsVisualization.setSelection([e]);
		}
		
		function barMouseOut(e) {
			barsVisualization.setSelection([{'row': null, 'column': null}]);
		}
		google.load("visualization", "1", {packages:["corechart"]});

		
		// graphTable
		$(function(){
			// TABEL STATICS		
			

			function initStatGraph(){
				
				$("table.statics").each(function() {
					var colors = [];
					$("table.statics thead th:not(:first)").each(function() {
						colors.push($(this).css("color"));
					});
					$('.flot-graph').remove();
					var $tmpTable = $(this).hide().clone(true).insertAfter(this);
					$tmpTable.graphTable({
						series: 'columns',
						position: 'replace',
						height: '200px',
						width: '100%',
						colors: colors
					}, {
						xaxis: {
							tickSize: 1
						}
					});
				});

				var previousPoint = null;
				$(".flot-graph").bind("plothover", function(event, pos, item) {
					$("#x").text(pos.x);
					$("#y").text(pos.y);

					if (item) {
						if (previousPoint != item.dataIndex) {
							previousPoint = item.dataIndex;

							$("#tooltip").remove();
							var x = item.datapoint[0],
								y = item.datapoint[1];

							showTooltip(item.pageX, item.pageY, "<b>" + item.series.label + "</b>: " + y);
						}
					}
					else {
						$("#tooltip").remove();
						previousPoint = null;
					}
				});
			}
			initStatGraph();
			// set window resize handler
			$(window).resize(initStatGraph);

			
			function showTooltip(x, y, contents) {
				$('<div id="tooltip">' + contents + '</div>').css({
					position: 'absolute',
					display: 'none',
					top: y + 5,
					left: x + 5
				}).appendTo("body").fadeIn("fast");
			}
			
			$('.flot-graph').before('<div class="space"></div>');
			
			
		});
	</script>
	
	<div class="wrapper">
		<h3>Visitors for the last 30 days</h2>
		<div id='barchart_div'></div>
		<!-- flot -->
		<table class="statics" style="width : 100%;"> 
			<thead> 
				<tr> 
					<th></th> 
					<th style="color : #00366f;">unique visitors</th> 
					<th style="color : #006ba0;">pageviews</th> 
				</tr> 
			</thead> 
			<tbody> 
				<?php echo $monthVisits2;?>
				
			</tbody> 
		</table>
		<div class="pie-column-holder">
		<div class="pie-chart-holder">
				<h3>Trafic Sources Overview</h2>
				<div id='piechart_div'></div>
			</div>
		<div class="total-values">
			<ul>
				<li><?php echo $totalCounter['visits'];?> <em>Visits</em></li>
				<li><?php echo $totalCounter['entranceBounceRate'];?>% <em>Bounce Rate</em></li>
				<li><?php echo $totalCounter['pageviews'];?> <em>Pagewies</em></li>
				<li><?php echo $totalCounter['avgTimeOnSite'];?> <em>Avg. Time on Site</em></li>
				<li><?php echo round($totalCounter['pageviews']/($totalCounter['visits'] || 1), 2);?> <em>Pages/Visit</em></li>					<li><?php echo $totalCounter['percentNewVisits'];?>% <em>New Visits</em></li>
			</ul>
		</div>
		</div>
		<h2>Top Traffic Sources</h2>
		<div class="table-column-holder">
			<div id='table_div'></div>
			<div id='table_div_key'></div>
		</div>
	</div>