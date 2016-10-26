<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN" "http://www.w3.org/TR/html4/loose.dtd">
<html>
<head>
<meta http-equiv="Content-Type" content="text/html; charset=utf-8">
<title>Upload BitmapData to server</title>
<script src="js/active_content.js" type="text/javascript"></script>
<script type="text/javascript">
function load_complete(id){
	alert(id);
}
</script>
</head>

<body>

<?

// Входящие переменные для флеш
$id = 5;
$upload_script = 'upload_script.php'; // скрипт обработки
$dir = 'upload'; // каталог для закрузки
$file_name = 'snapshot'; // имя файла на выходе без расширения (расширение добавиться автоматически в зависимости от выбранного типа файла)
$q = 100; // качество на выходе
$max_width = 200; // максимальная ширина картинки

?>

<script type="text/javascript">
AC_FL_RunContent('quality','high','pluginspage','http://www.adobe.com/shockwave/download/download.cgi?P1_Prod_Version=ShockwaveFlash','menu','false','width','225','height','261','title','','movie','uploader','src','uploader','flashvars','<? echo 'id='.$id.'&upload_script='.$upload_script.'&dir='.$dir.'&file_name='.$file_name.'&q='.$q.'&max_width='.$max_width ?>');
</script>
<noscript>
<object classid="clsid:D27CDB6E-AE6D-11cf-96B8-444553540000" width="225" height="261" title="">
<param name="movie" value="uploader.swf">
<param name="quality" value="high">
<embed quality="high" pluginspage="http://www.adobe.com/shockwave/download/download.cgi?P1_Prod_Version=ShockwaveFlash" type="application/x-shockwave-flash" width="225" height="261" src="uploader.swf" menu="false" flashvars="<? echo 'id='.$id.'&upload_script='.$upload_script.'&dir='.$dir.'&file_name='.$file_name.'&q='.$q.'&max_width='.$max_width ?>">
</embed>
</object>
</noscript>

</body>
</html>
