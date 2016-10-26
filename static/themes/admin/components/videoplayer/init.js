function mediaVideoPlayerInit(video, image, title)
{    
	var so = new SWFObject("/components/videoplayer/videoplayer.swf", "mymovie", "540", "380", "9","#333333");
	so.addParam("menu", "false");
	so.addParam("allowfullscreen", "true");
	so.addParam("allowScriptAccess", "always");
	so.addVariable("setting", "9");
	so.addVariable("playerAutoHide", "yes");
			
	so.addVariable("videoWidth","540");
	so.addVariable("videoHeight","380");

	so.addVariable("imagePath",image);
	so.addVariable("videoPath",video);
	so.addVariable("videoDefaultQuality","small");
			
	so.addVariable("playerNavigations","ply,sek,vol,ful,txt");
			
	so.addVariable("videoAutoStart","no");
	so.addVariable("reflection","no");
	so.addParam('wmode', 'opaque');
			
	so.addVariable("titleVerticalSpace","75");
	so.addVariable("videoTitle", "<b>"+title+"</b>");
			
	so.addVariable("videoDescription","This website is powered by <b>MediRocket</b> - Marketing solutions for Medical Industry. <b><a href='http://medirocket.com'>www.medirocket.com</a></b>");
	so.write("flashcontent");
}