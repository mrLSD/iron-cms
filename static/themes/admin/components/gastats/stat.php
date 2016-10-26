<?
error_reporting(15);
include("config.php");
include("gapi.class.php");

	$ga = new gapi($u,$p);
	

	$ga->requestReportData($id,array('day','month','year'),array('visitors','visits','pageviews', 'avgTimeOnSite', 'entranceBounceRate', 'percentNewVisits'),array('year','month'),null,$date1MonthStart, $date1MonthFinish,1,1000);
	$output= array();
	
	$totalCounter = array(
		"visits" => $ga->getVisits(),
		"visitors" => $ga->getVisitors(),
		"pageviews" => $ga->getPageviews(),
		"avgTimeOnSite" => date("H:i:s", mktime(0,0,round($ga->getAvgTimeOnSite()),0,0,0)),
		"entranceBounceRate" => round($ga->getEntranceBounceRate(), 2),
		"percentNewVisits" => round($ga->getPercentNewVisits(), 2),
	);
	
	$i = 0;
	foreach($ga->getResults() as $result)
	{
		$d = $result;
		$visitors = $result->getVisitors();
		$pageviews = $result->getPageviews();
		$visits = $result->getVisits();

		$output[] = "[\"" . $d. "\"," . $visits . "]";
		$output2[] = "<tr><th>" . $i. "</th><td>" . $visits . "</td><td>" . $pageviews . "</td></tr>";
		$i++;
	}
	$monthVisits = implode(",\n", $output);
	$monthVisits2 = implode("\n", $output2);
	
	
	
	$ga->requestReportData($id,array('country'),array('visits'),'-visits',null,$date1MonthStart, $date1MonthFinish,1,$countryRows);
	$output= array();
	foreach($ga->getResults() as $result)
	{
		$country=$result->getCountry(); //страна
		$visits=$result->getVisits(); //кол-во посещений
		$output[]="[\"" . $country . "\"," . $visits . "]";
	}
	$monthCountrys = implode(",\n", $output);
	
	$ga->requestReportData($id,array('keyword'),array('visits'),'-visits',null,$date1MonthStart, $date1MonthFinish,1,$cityRows);
	$output= array();
	$total_visits=$ga->getVisits();
	foreach($ga->getResults() as $result)
	{
		$keyword=$result->getKeyword();
		$visits=$result->getVisits();
		$output[]="[\"" . $keyword . "\"," . $visits . ", '" .  round($visits / $total_visits * 100, 2) . "%']";
	}
	$monthKeyword =  implode(",\n", $output);
	

	$ga->requestReportData($id,array('source'),array('visits'),'-visits',null,$date1MonthStart, $date1MonthFinish,1,$cityRows);
	$output= array();
	$total_visits=$ga->getVisits();
	foreach($ga->getResults() as $result)
	{
		$source=$result->getSource();
		$visits=$result->getVisits();
	
		$output[]="[\"" . $source . "\"," . $visits . ", '" .  round($visits / $total_visits * 100, 2) . "%']";
	}
	$monthSource =  implode(",\n", $output);

?>