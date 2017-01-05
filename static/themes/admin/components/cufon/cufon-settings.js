function initCufon() {
	Cufon.replace('.headbox h1, .signup h3, .reviews h3, .content-area h2, .question h3 ', { fontFamily: 'DroidSerif', hover: true });
}
$(document).ready(function(){
	initCufon();
});