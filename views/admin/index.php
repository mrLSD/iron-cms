<?php

/*Yii::app()->clientScript->registerScript('mce', '
$.get("/admin",function(data){
});
',  CClientScript::POS_LOAD); */
?>				
				<div class="page-title">

					<h2>Welcome Admin,</h2>
				</div>
				
					<ul class="dashboard-menu">
						<li><a href="<?php echo Yii::app()->createUrl($this->module->ID.'/pages/')?>" class="create">Create a Page</a></li>
						<li><a href="<?php echo Yii::app()->createUrl($this->module->ID.'/gallery/')?>" class="pictures">Post B/A pictures</a></li>
						<li><a href="<?php echo Yii::app()->createUrl($this->module->ID.'/review/manage/')?>" class="reviews">Post Reviews</a></li>
						<li><a href="<?php echo Yii::app()->createUrl($this->module->ID.'/media/manage/')?>" class="media">Post Media</a></li>
					</ul>
					<!-- start content-box -->
					<div class="content-box">
						<!-- start box-header -->
						<div class="box-header">
							<strong>Analytics</strong>
						</div>

						<!-- end box-header -->
						<div class="box-content">
							<?php //include('components/gastats/index.php');?>
						</div>
					</div>
					<!-- end content-box -->
				