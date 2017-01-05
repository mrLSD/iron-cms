<?
//учетная запись GA

$u="ruslan.kesheshyan@gmail.com";
$p="K700557051";
//$id="48252886";
//$id="43815962";
$id="51266134";


//текущая дата
$currentdate=date("Ymd");
//дата, начиная с которой необходимо получить данные из GA для отчета. Формат YYYY-MM-DD
$datestart="2011-05-01";
//дата, заканчивая которой
//$datefinish="";
//или вычисляем дату - конец предыдущего месяца
$currentday=date("d");$currentmonth=date("m");$currentyear=date("Y");
$datefinish=date("Y-m-d",mktime(0,0,0,$currentmonth,0,$currentyear));

//дата 3 месяца назад
$date3MonthStart=date("Y-m-d",mktime(0,0,0,$currentmonth-3,$currentday-1,$currentyear));
$date3MonthFinish=date("Y-m-d",mktime(0,0,0,$currentmonth,$currentday-1,$currentyear));

//дата месяц назад
$date1MonthStart=date("Y-m-d",mktime(0,0,0,$currentmonth-1,$currentday-1,$currentyear));
$date1MonthFinish=date("Y-m-d",mktime(0,0,0,$currentmonth,$currentday-1,$currentyear));

//количество стран
$countryRows=3;
//количество городов
$cityRows=10;


//csv-файл для отчета Посетители
$visitorsCSV="visitors.csv";
//csv-файл для отчета Посетители за посл. 3 месяца
$visitors3CSV="visitors_3.csv";
//csv-файл для отчета География по странам
$countryCSV="country.csv";
//csv-файл для отчета География по городам
$cityCSV="city.csv";
//файл со статистикой до начала использования GA. Формат: дата;посетители;просмотры
//$addFile="default.csv";
$addFile=false;

//полный пусть к директории со скриптом (слэш в конце обязателен!)
$path="/home/gatest/www/";

?>