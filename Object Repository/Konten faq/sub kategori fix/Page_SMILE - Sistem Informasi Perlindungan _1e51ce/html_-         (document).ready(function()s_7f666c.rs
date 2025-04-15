<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>html_-         (document).ready(function()s_7f666c</name>
   <tag></tag>
   <elementGuidId>237d13f7-3d02-43b3-9218-085427e4b9d6</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>html</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//*/text()[normalize-space(.)='']/parent::*</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>html</value>
      <webElementGuid>12e759a1-d77d-412f-bb0f-4ed46c774e5e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xmlns</name>
      <type>Main</type>
      <value>http://www.w3.org/1999/xhtml</value>
      <webElementGuid>951204ef-e977-49e9-9e53-460044c0010f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xml:lang</name>
      <type>Main</type>
      <value>en</value>
      <webElementGuid>52a28083-ca1c-4cf5-ab8b-4d16b437af8d</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>data-select2-id</name>
      <type>Main</type>
      <value>141</value>
      <webElementGuid>f44f4a3e-fa89-450a-a6ac-944ee60859c5</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
  
   - 
  
  
	
    

$(document).ready(function(){	
	setTimeout(function(){
		$('#loading').hide();
		$('#loading-mask').hide();
	}, 250);
});
function alertError(msg){
	window.parent.Ext.MessageBox.show({
	   title: 'Perhatian',
	   msg: msg,
	   buttons: window.parent.Ext.MessageBox.OK,
	   icon: 'x-message-box-error'
   });	
}
function alert(msg){
	window.parent.Ext.MessageBox.alert('Perhatian', msg);
}
function alertMsg(msg){
	window.parent.Ext.MessageBox.alert('Informasi', msg);
}
window.notificationCountAlertNew = window.notificationCountAlertNew || 0;
function showAutoCloseAlert(msg, typeAlert = 'info', title = &quot;Perhatian&quot;, countClose = 4000) {
	console.log(window.notificationCountAlertNew);
	let baseColorAlert = '#4CAF50'
	let backGroundColorAlert = '#f0f8ff'
	let textColroAlert = '#000'
	switch (typeAlert) {
		case 'info':
			baseColorAlert = '#3A87AD'
			backGroundColorAlert = '#f0f8ff'
			textColroAlert = '#000'
			break;
		case 'success':
			baseColorAlert = '#4CAF50'
			backGroundColorAlert = '#f0f8ff'
			textColroAlert = '#000'
			break;
		case 'warning':
			baseColorAlert = '#C09853'
			backGroundColorAlert = '#f0f8ff'
			textColroAlert = '#000'
			break;
		case 'error':
			baseColorAlert = '#B94A48'
			backGroundColorAlert = '#f0f8ff'
			textColroAlert = '#000'
			break;
		default:
			baseColorAlert = '#4CAF50'
			backGroundColorAlert = '#f0f8ff'
			textColroAlert = '#000'
			break;
	}
	var baseY = 20;
	var notificationHeight = 110;
	var currentY = baseY + (window.notificationCountAlertNew * notificationHeight);
	if (window.notificationCountAlertNew+1 &lt; 6) {
		var win = window.parent.Ext.create('Ext.window.Window', {
			title: title,
			html: msg,
			width: 450,
			height: 100,
			closable: true,
			bodyStyle: `padding: 10px; background-color: ${backGroundColorAlert}; color: #000; border-radius: 8px;`,
			autoShow: true,
			modal: false,
			shadow: true,
			resizable: false,
			draggable: false,
			x: (window.parent.Ext.getBody().getViewSize().width - 450) / 2,
			y: currentY,
			style: {
				borderColor: `${baseColorAlert} !important`,
				borderStyle: 'solid',
				borderWidth: '2px',
				borderRadius: '8px'
			},
			header: {
				style: {
					borderColor: `${baseColorAlert} !important`,
					backgroundColor: `${baseColorAlert}`,
					color: '#fff',
					borderRadius: '8px 8px 0 0'
				},
			},
			listeners: {
				afterrender: function(win) {
					// Remove the background color of the existing close button
					var closeButton = win.header.el.down('.x-tool-close');
					if (closeButton) {
						closeButton.setStyle({
							backgroundColor: 'transparent', // Remove background color
							border: 'none', // Remove border if any
							color: '#fff' // Set icon color
						});
					}

					// Close the window after a custom delay
					window.parent.Ext.defer(function() {
						win.close();
						if (window.notificationCountAlertNew > 0) {
							window.notificationCountAlertNew--; // Decrement the counter only if it's greater than 0
						}
					}, countClose); // Time in milliseconds (4 seconds)
				},
				close: function(win) {
					// Trigger when the X button is clicked
					if (window.notificationCountAlertNew > 0) {
						window.notificationCountAlertNew--; // Decrement the counter only if it's greater than 0
					}
				}
			}
		});

		window.notificationCountAlertNew++;
	}
}

#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}






    .saved_message_success {
        font-weight:bold;font-size:12px;color:black;padding-bottom:30px;padding-top:30px;
    }
    .saved_message_failed {
        font-weight:bold;font-size:12px;color:red;padding-bottom:30px;padding-top:30px;
    }

    div.DialogMask
    {
        padding: 10px;
        position: fixed;
        top: 0;
        right: 0;
        bottom: 0;
        left: 0;
        z-index: 50;
        background-color: #606060;
        filter: progid:DXImageTransform.Microsoft.Alpha(Opacity=50);
        -moz-opacity: .5;
        opacity: .5;
    }





  



    //logger.disableLogger();
    logger.enableLogger();
	$(document).ready(function(){
            $(&quot;input[type=text]&quot;).keyup(function(){
                    //$(this).val($(this).val().toUpperCase());
            });
            
            $('.kategori').keypress(function (e) {
                console.log(&quot;keypress button: e.key=&quot;+e.key+&quot; - e.code=&quot;+e.code+&quot; - e.which=&quot;+e.which);
                /*if (e.which === 44 || e.key === ',' || e.key === '&lt;' || e.key === '.' || e.key === '>' || e.key === '?' ||
                    e.key === '/' || e.key === ':' || e.key === ';' || e.key === '&quot;' || e.key === &quot;'&quot; || e.key === '{' ||
                    e.key === '[' || e.key === '}' || e.key === ']' || e.key === '|' || e.key === '\\' || e.key === '`' ||
                    e.key === '~' || e.key === '!' || e.key === '@' || e.key === '#' || e.key === '$' || e.key === '%' ||
                    e.key === '^' || e.key === '&amp;' || e.key === '*' || e.key === '(' || e.key === ')' || e.key === '_' ||
                    e.key === '-' || e.key === '+' || e.key === '=') {*/
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:'&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                    //alert(&quot;comma detected1&quot;);
                /*} else {
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replaceAll(/[`~!@#$%^&amp;*()_|+\=?;:'&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                }*/
            }).keyup(function (e) {
                /*if (e.which === 44 || e.key === ',' || e.key === '&lt;' || e.key === '.' || e.key === '>' || e.key === '?' ||
                    e.key === '/' || e.key === ':' || e.key === ';' || e.key === '&quot;' || e.key === &quot;'&quot; || e.key === '{' ||
                    e.key === '[' || e.key === '}' || e.key === ']' || e.key === '|' || e.key === '\\' || e.key === '`' ||
                    e.key === '~' || e.key === '!' || e.key === '@' || e.key === '#' || e.key === '$' || e.key === '%' ||
                    e.key === '^' || e.key === '&amp;' || e.key === '*' || e.key === '(' || e.key === ')' || e.key === '_' ||
                    e.key === '-' || e.key === '+' || e.key === '=') {*/
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:'&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                    //alert(&quot;comma detected1&quot;);
                /*} else {
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replaceAll(/[`~!@#$%^&amp;*()_|+\=?;:'&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                }*/
            });
            
            $('.subkategori').keypress(function (e) {
                console.log(&quot;keypress button: e.key=&quot;+e.key+&quot; - e.code=&quot;+e.code+&quot; - e.which=&quot;+e.which);
                /*if (e.which === 44 || e.key === ',' || e.key === '&lt;' || e.key === '.' || e.key === '>' || e.key === '?' ||
                    e.key === '/' || e.key === ':' || e.key === ';' || e.key === '&quot;' || e.key === &quot;'&quot; || e.key === '{' ||
                    e.key === '[' || e.key === '}' || e.key === ']' || e.key === '|' || e.key === '\\' || e.key === '`' ||
                    e.key === '~' || e.key === '!' || e.key === '@' || e.key === '#' || e.key === '$' || e.key === '%' ||
                    e.key === '^' || e.key === '&amp;' || e.key === '*' || e.key === '(' || e.key === ')' || e.key === '_' ||
                    e.key === '-' || e.key === '+' || e.key === '=') {*/
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:'&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                    //alert(&quot;comma detected1&quot;);
                /*} else {
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:'&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                }*/
            }).keyup(function (e) {
                /*if (e.which === 44 || e.key === ',' || e.key === '&lt;' || e.key === '.' || e.key === '>' || e.key === '?' ||
                    e.key === '/' || e.key === ':' || e.key === ';' || e.key === '&quot;' || e.key === &quot;'&quot; || e.key === '{' ||
                    e.key === '[' || e.key === '}' || e.key === ']' || e.key === '|' || e.key === '\\' || e.key === '`' ||
                    e.key === '~' || e.key === '!' || e.key === '@' || e.key === '#' || e.key === '$' || e.key === '%' ||
                    e.key === '^' || e.key === '&amp;' || e.key === '*' || e.key === '(' || e.key === ')' || e.key === '_' ||
                    e.key === '-' || e.key === '+' || e.key === '=') {*/
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:'&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                    //alert(&quot;comma detected1&quot;);
                /*} else {
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:'&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                }*/
            });

            $('select.tipe_input').select2();
            $('select#kategori').select2();
            $('select.kategori').select2();
            $('select#tipe').select2();
            $('select.tipe').select2();
            $('select.seq').select2();
            $('select.status').select2();
            $('select').select2();

            $(window).bind(&quot;resize&quot;, function(){
                resize();
            });
            resize();
            // filter();
            /** list checkbox npp */
            window.list_npp = [];
	});
	var asyncPreloadStart;

	function asyncPreloadX(state){
            if (state == true) {
                asyncPreloadStart = setInterval(function() {
                    $('#loading').show();
                    $('#loading-mask').show();
                }, 50);
            } else {
                $('#loading').hide();
                $('#loading-mask').hide();
                clearInterval(asyncPreloadStart);
            }
	}
        
        function preloadMask(param_true_false) {
            if (param_true_false==true) {
                if ($('.DialogMask').length == 0) {
                    $('body').append('&lt;div class=&quot;DialogMask&quot; style=&quot;z-index:20000;background-color:black;opacity:0.5;width:100%;height:100vh;display:flex;justify-content:center;align-items:center;&quot;>'+
                                        '&lt;img style=&quot;width:40px;height:auto;&quot; src=&quot;http://172.28.108.46:5454/smile/images/loading.gif&quot; />'+
                                        '&lt;span style=&quot;display:inline-block;position:fixed;bottom:130px;right:0px;left:0px;text-align:center;color:white;font-weight:bold;font-size:12px;font-style:italic;padding-left:10px;&quot;>in progress...&lt;/span>'+
                                     '&lt;/div>');
                }
                $('.DialogMask').show();
            } else {
                $('.DialogMask').hide();
            }
        }
	
	function getValue(val){
            return val == null || val == undefined ? '' : val;
	}

	function search_by_changed(){
            $(&quot;#search_txt&quot;).val(&quot;&quot;);
            $(&quot;#search_txt2&quot;).val(&quot;&quot;);
	}

	function isNumber(evt) {
            evt = (evt) ? evt : window.event;
            var charCode = (evt.which) ? evt.which : evt.keyCode;
            if (charCode > 31 &amp;&amp; (charCode &lt; 48 || charCode > 57)) {
                return false;
            }
            return true;
	}
	
	function validateDigit(evt) {
            var theEvent = evt || window.event;
            var key = theEvent.keyCode || theEvent.which;
            key = String.fromCharCode(key);
            var regex = /[0-9]|\./;
            if (!regex.test(key)) {
                theEvent.returnValue = false;
                if (theEvent.preventDefault)
                    theEvent.preventDefault();
            }
        }
	
	function Comma(Num) { //function to add commas to textboxes
            Num += '';
            Num = Num.replace(',', '');
            Num = Num.replace(',', '');
            Num = Num.replace(',', '');
            Num = Num.replace(',', '');
            Num = Num.replace(',', '');
            Num = Num.replace(',', '');
            x = Num.split('.');
            x1 = x[0];
            x2 = x.length > 1 ? '.' + x[1] : '';
            var rgx = /(\d+)(\d{3})/;
            while (rgx.test(x1))
                            x1 = x1.replace(rgx, '$1' + ',' + '$2');
            return x1 + x2;
	}

	function isNumberKey(evt) {
            var theEvent = evt || window.event;
            var key = theEvent.keyCode || theEvent.which;
            key = String.fromCharCode(key);
            if (key.length == 0)
                    return;
            var regex = /^[0-9\b]+$/;
            if (!regex.test(key)) {
                    theEvent.returnValue = false;
                    if (theEvent.preventDefault)
                            theEvent.preventDefault();
            }
	}

	function resize(){
            $(&quot;#div_container&quot;).width($(&quot;#div_dummy&quot;).width());

            $(&quot;#div_header&quot;).width($(&quot;#div_dummy&quot;).width());
            $(&quot;#div_body&quot;).width($(&quot;#div_dummy&quot;).width());
            $(&quot;#div_footer&quot;).width($(&quot;#div_dummy&quot;).width());

            $(&quot;#div_filter&quot;).width(0);
            $(&quot;#div_data&quot;).width(0);
            $(&quot;#div_page&quot;).width(0);
            $(&quot;#div_footer&quot;).width(0);

            $(&quot;#div_filter&quot;).width($(&quot;#div_dummy_data&quot;).width());
            $(&quot;#div_data&quot;).width($(&quot;#div_dummy_data&quot;).width());
            $(&quot;#div_page&quot;).width($(&quot;#div_dummy_data&quot;).width());
            $(&quot;#div_footer&quot;).width($(&quot;#div_dummy_data&quot;).width());

            $(&quot;#div_container&quot;).css('max-height', $(window).height());

            let margin_height = 40;
            let filter_height = $('#div_body').height() - $('#div_data').height() + margin_height;
            $('#div_data').css('max-height', $(window).height() - $('#div_header').height() - $('#div_page').height() - $('#div_footer').height() - filter_height);
	}
	
	function confirmation(title, msg, fnYes, fnNo) {
            window.parent.Ext.Msg.show({
                title: title,
                msg: msg,
                buttons: window.parent.Ext.Msg.YESNO,
                icon: window.parent.Ext.Msg.QUESTION,
                fn: function(btn) {
                    if (btn === 'yes') {
                        fnYes();
                    } else {
                        fnNo();
                    }
                }
            });
	}

	function showFormReload(mypage, myname, w, h, scroll) {
            var openwin = window.parent.Ext.create('Ext.window.Window', {
                title: myname,
                collapsible: true,
                animCollapse: true,
                maximizable: true,
                closable: true,
                width: w,
                height: h,
                minWidth: w,
                minHeight: h,
                layout: 'fit',
                modal: true,
                html: '&lt;iframe src=&quot;' + mypage + '&quot;  frameborder=&quot;0&quot; style=&quot;border:0; height:100%; width:100%; overflow-y:hidden; overflow-x:hidden; overflow:hidden;&quot; scrolling=&quot;no&quot;>&lt;/iframe>',
                listeners: {
                    close: function () {
                        resubmit();
                    },
                        destroy: function (wnd, eOpts) {
                    }
                }
            });
            openwin.show();
            return openwin;
	}

	function showFormRefilter(mypage, myname, w, h, scroll) {
            var openwin = window.parent.Ext.create('Ext.window.Window', {
                title: myname,
                collapsible: true,
                animCollapse: true,
                maximizable: true,
                closable: true,
                width: w,
                height: h,
                minWidth: w,
                minHeight: h,
                layout: 'fit',
                modal: true,
                html: '&lt;iframe src=&quot;' + mypage + '&quot;  frameborder=&quot;0&quot; style=&quot;border:0; height:100%; width:100%; overflow-y:hidden; overflow-x:hidden; overflow:hidden;&quot; scrolling=&quot;no&quot;>&lt;/iframe>',
                listeners: {
                    close: function () {
                        filter();
                    },
                        destroy: function (wnd, eOpts) {
                    }
                }
            });
            openwin.show();
            return openwin;
	}

	function showForm(mypage, myname, w, h, scroll) {
            var openwin = window.parent.Ext.create('Ext.window.Window', {
                title: myname,
                collapsible: true,
                animCollapse: true,
                maximizable: true,
                closable: true,
                width: w,
                height: h,
                minWidth: w,
                minHeight: h,
                layout: 'fit',
                modal: true,
                html: '&lt;iframe src=&quot;' + mypage + '&quot;  frameborder=&quot;0&quot; style=&quot;border:0; height:100%; width:100%; overflow-y:hidden; overflow-x:hidden; overflow:hidden;&quot; scrolling=&quot;no&quot;>&lt;/iframe>',
                listeners: {
                    close: function () {
                    },
                        destroy: function (wnd, eOpts) {
                    }
                }
            });
            openwin.show();
            return openwin;
	}

	function getBase64(file) {
            return new Promise((resolve, reject) => {
		const reader = new FileReader();
		reader.readAsDataURL(file);
		reader.onload = () => resolve(reader.result);
		reader.onerror = error => reject(error);
            });
	}
        
        function enableSelect2EditFailed(data_ke) {
            console.log(&quot;enableSelect2EditFailed: &quot;+data_ke);
            $(&quot;#tipe_input&quot;+data_ke+&quot;, #kategori&quot;+data_ke+&quot;, #seq&quot;+data_ke+&quot;, #status&quot;+data_ke).removeAttr('disabled').removeAttr('readonly');
            $(&quot;#tipe_input&quot;+data_ke+&quot;, #kategori&quot;+data_ke+&quot;, #seq&quot;+data_ke+&quot;, #status&quot;+data_ke).select2({allowClear: true});
            $(&quot;#tipe_input&quot;+data_ke+&quot;, #kategori&quot;+data_ke+&quot;, #seq&quot;+data_ke+&quot;, #status&quot;+data_ke).prop('disabled', false);
            //$(&quot;#tipe_input&quot;+data_ke+&quot; select:first-of-type, #kategori&quot;+data_ke+&quot; select:first-of-type, #seq&quot;+data_ke+&quot; select:first-of-type, #status&quot;+data_ke+&quot; select:first-of-type&quot;).prop('disabled', false);
            //$(&quot;#tipe_input&quot;+data_ke+&quot;:first-of-type, #kategori&quot;+data_ke+&quot;:first-of-type, #seq&quot;+data_ke+&quot;:first-of-type, #status&quot;+data_ke+&quot;:first-of-type&quot;).prop('disabled', false);
            $(&quot;#tipe_input&quot;+data_ke+&quot;, #kategori&quot;+data_ke+&quot;, #seq&quot;+data_ke+&quot;, #status&quot;+data_ke).on('select2:select', function(e) {
                $(this).nextAll('select').first().prop('disabled', false);
            });

            /*$('select').select2();
            $('select').prop(&quot;disabled&quot;, true);
            $('select').eq(0).prop(&quot;disabled&quot;, false);
            $('select').on('select2:select', function(e) {
              var index = $(this).index('select');
              $('select').eq(index + 1).prop(&quot;disabled&quot;, false);
            });*/

            $(&quot;#tipe_input&quot;+data_ke+&quot;, #kategori&quot;+data_ke+&quot;, #seq&quot;+data_ke+&quot;, #status&quot;+data_ke).select2(&quot;enable&quot;, true);
        }
	
	async function save(parameter=&quot;&quot;){
            if (parameter===&quot;new&quot;) {
		confirmation('Simpan Data', 'Yakin untuk simpan data?', async function(){
                    empty_field = 0;
                    length_field_100 = 0;
                    $('input[type=text]').each(function() {
                        if($(this).val() == '') {
                           empty_field += 1;
                           $(this).css('background-color' , '#f8c2b6'); //FF0000
                        } else {
                            if ($(this).val().length > 100) {
                                length_field_100++;
                                $(this).css('background-color' , '#f8c2b6'); //FF0000
                            }
                        }
                    });
                    if (empty_field > 0) return alert('Tidak boleh kosong');
                    if (length_field_100 > 0) return alert('Field size tidak boleh melebihi 100 karakter. Length saat ini: '+length_field_100);
                    
                    const form = $('form')[0]; 
                    let formData = new FormData(form);
                    formData.append('tipe', 'save_new');

                    preloadMask(true);
                    $.ajax({
                        type: 'POST',
                        url: &quot;http://172.28.108.46:5454/smile/mod_ec/ajax/as1031_action.php?&quot;+Math.random(),
                        data: formData ? formData : form.serialize(),
                        contentType : false,
                        processData: false,
                        success: function(datas){
                            //alert(datas);
                            console.log(&quot;datas&quot;,datas);
                            jdata = JSON.parse(datas);
                            console.log(jdata);
                            console.log(&quot;jdata.length: &quot;+Object.keys(jdata).length);
                            if (Object.keys(jdata).length==1) {
                                if (jdata[0].ret == 1){
                                    alert(jdata[0].msg);
                                    //location.replace('http://&lt; ?=$HTTP_HOST;?>/mod_ec/form/as1031.php?former_kategori_subkategori=&lt; ?=$former_kategori_subkategori?>'+&quot;&amp;former_search_txt=&lt; ?=$former_search_txt?>&quot;+&quot;&amp;former_search_txt2=&lt; ?=$former_search_txt2?>&quot;+&quot;&amp;former_search_txt3=&lt; ?=$former_search_txt3?>&quot;);
                                    //location.replace('http://172.28.108.46:5454/smile/mod_ec/form/as1031.php?former_kategori_subkategori=Y'+&quot;&amp;former_search_txt=&quot;+&quot;&amp;former_search_txt2=&quot;+&quot;&amp;sess=div-container-table&quot;+&quot;&amp;selected_data=&quot;+&quot;&amp;status_crud=sukses&quot;);
                                    location.replace('http://172.28.108.46:5454/smile/mod_ec/form/as1031.php?former_kategori_subkategori=Y'+&quot;&amp;former_search_txt=&quot;+&quot;&amp;former_search_txt2=&quot;);
                                } else {
                                    alert(jdata[0].msg);
                                }
                            } else {
                                $(&quot;#saved_message&quot;).html(&quot;&quot;);
                                $(&quot;#saved_message&quot;).show();
                                var return_message = &quot;&quot;;
                                var failed = &quot;&quot;;
                                $.each(jdata, function(idx, val) {
                                    return_message = return_message + &quot;Data ke-&quot; + val.ke + &quot;: &quot; + val.msg + &quot;\n\r&lt;br/>&quot;;
                                    if (val.ret==&quot;1&quot;) {
                                        $(&quot;#saved_message&quot;).append(&quot;&lt;span class='saved_message_success'>idx ke: &quot;+idx+&quot;, val ke: &quot;+val.ke+&quot;, val ret: &quot;+val.ret+&quot;, val msg: &quot;+val.msg+&quot;, val no urut: &quot;+val.noUrut+&quot;, val input: &quot;+val.input+&quot;, val status: &quot;+val.status+&quot;&lt;span>&lt;br/>&lt;br/>                        \n\n\n\n&quot;);
                                        $(&quot;#saved_message&quot;).append(&quot;\n\r&quot;);
                                        $(&quot;#saved_message&quot;).append(&quot;\n\r&quot;);
                                        
                                        div_color = &quot;&lt;div style='z-index:2;background: linear-gradient(159deg, rgba(173,223,173,1) 0%, rgba(50,205,50,1) 100%);opacity:0.3;position:absolute;border-radius:10px;left:0px;top:0px;right:0px;bottom:0px;'>&lt;/div>&quot;;
                                        if (idx==0) {
                                            $(&quot;.konten_form0&quot;).append(div_color);
                                            $(&quot;.konten_form0 input[type=text]&quot;).attr('disabled','disabled');
                                            $(&quot;.konten_form0 input[type=text]&quot;).attr('readonly','readonly');
                                            $(&quot;.konten_form0 select&quot;).select2(&quot;enable&quot;, false);
                                        } else {
                                            $(&quot;.konten_form&quot;+(idx+1)).append(div_color);
                                            $(&quot;.konten_form&quot;+(idx+1)+&quot; input[type=text]&quot;).attr('disabled','disabled');
                                            $(&quot;.konten_form&quot;+(idx+1)+&quot; input[type=text]&quot;).attr('readonly','readonly');
                                            $(&quot;.konten_form&quot;+(idx+1)+&quot; select&quot;).select2(&quot;enable&quot;, false);
                                        }
                                    } else {
                                        $(&quot;#saved_message&quot;).append(&quot;&lt;span class='saved_message_failed'>idx ke: &quot;+idx+&quot;, val ke: &quot;+val.ke+&quot;, val ret: &quot;+val.ret+&quot;, val msg: &quot;+val.msg+&quot;, val no urut: &quot;+val.noUrut+&quot;, val input: &quot;+val.input+&quot;, val status: &quot;+val.status+&quot;&lt;span>&lt;br/>&lt;br/>                        \n\n\n\n&quot;);
                                        $(&quot;#saved_message&quot;).append(&quot;\n\r&quot;);
                                        $(&quot;#saved_message&quot;).append(&quot;\n\r&quot;);
                                        failed += failed + &quot;Data ke-&quot; + val.ke + &quot;: &quot; + val.msg + &quot;\n\r&lt;br/>&quot;;
                                        
                                        if (idx==0) {
                                            $(&quot;.konten_form0 input[type=text]&quot;).removeAttr('disabled');
                                            $(&quot;.konten_form0 input[type=text]&quot;).removeAttr('readonly');
                                            //$(&quot;.konten_form0 select&quot;).select2(&quot;enable&quot;, true);
                                        } else {
                                            $(&quot;.konten_form&quot;+(idx+1)+&quot; input[type=text]&quot;).removeAttr('disabled');
                                            $(&quot;.konten_form&quot;+(idx+1)+&quot; input[type=text]&quot;).removeAttr('readonly');
                                            //$(&quot;.konten_form&quot;+(idx+1)+&quot; select&quot;).select2(&quot;enable&quot;, true);
                                        }
                                    }
                                });
                                alert(return_message);
                                if (failed==&quot;&quot;) {
                                    //location.replace('http://&lt; ?=$HTTP_HOST;?>/mod_ec/form/as1031.php?former_kategori_subkategori=&lt; ?=$former_kategori_subkategori?>'+&quot;&amp;former_search_txt=&lt; ?=$former_search_txt?>&quot;+&quot;&amp;former_search_txt2=&lt; ?=$former_search_txt2?>&quot;+&quot;&amp;former_search_txt3=&lt; ?=$former_search_txt3?>&quot;);
                                    //location.replace('http://&lt; ?=$HTTP_HOST;?>/mod_ec/form/as1031.php?former_kategori_subkategori=&lt; ?=$former_kategori_subkategori?>'+&quot;&amp;former_search_txt=&lt; ?=$former_search_txt?>&quot;+&quot;&amp;former_search_txt2=&lt; ?=$former_search_txt2?>&quot;+&quot;&amp;sess=&lt; ?=$former_sess_table?>&quot;+&quot;&amp;selected_data=&lt; ?=$former_selected_data?>&quot;+&quot;&amp;status_crud=sukses&quot;);
                                    location.replace('http://172.28.108.46:5454/smile/mod_ec/form/as1031.php?former_kategori_subkategori=Y'+&quot;&amp;former_search_txt=&quot;+&quot;&amp;former_search_txt2=&quot;);
                                }
                            }
                            
//                            if (jdata.ret == 1){
//                                alert(jdata.msg);
//                                //location.replace('http://&lt; ?= $HTTP_HOST; ?>/mod_ec/form/as1031.php?former_kategori_subkategori=&lt; ?=$former_kategori_subkategori?>'+&quot;&amp;former_search_txt=&lt; ?=$former_search_txt?>&quot;+&quot;&amp;former_search_txt2=&lt; ?=$former_search_txt2?>&quot;+&quot;&amp;former_search_txt3=&lt; ?=$former_search_txt3?>&quot;);
//                            } else {
//                                alert(jdata.msg);
//                            }
                            preloadMask(false);
                        },
                        complete: function(){
                            preloadMask(false);
                        },
                        error: function(){
                            alert(&quot;Terjadi kesalahan, coba beberapa saat lagi!&quot;);
                            preloadMask(false);
                        }
                    });
		}, function(){});
            } else if (parameter===&quot;edit&quot;) {
		confirmation('Edit Data', 'Yakin untuk edit data?', async function(){
                    empty_field = 0;
                    length_field_100 = 0;
                    $('input[type=text]').each(function() {
                        if($(this).val() == '') {
                           empty_field += 1;
                           $(this).css('background-color' , '#f8c2b6'); //FF0000
                        } else {
                            if ($(this).val().length > 100) {
                                length_field_100++;
                                $(this).css('background-color' , '#f8c2b6'); //FF0000
                            }
                        }
                    });
                    if (empty_field > 0) return alert('Tidak boleh kosong');
                    if (length_field_100 > 0) return alert('Field size tidak boleh melebihi 100 karakter. Length saat ini: '+length_field_100);
                    
                    const form = $('form')[0]; 
                    let formData = new FormData(form);
                    formData.append('tipe', 'save_edit');

                    preloadMask(true);
                    $.ajax({
                        type: 'POST',
                        url: &quot;http://172.28.108.46:5454/smile/mod_ec/ajax/as1031_action.php?&quot;+Math.random(),
                        data: formData ? formData : form.serialize(),
                        contentType : false,
                        processData: false,
                        success: function(datas){
                            //alert(datas);
                            console.log(&quot;datas&quot;,datas);
                            sessionStorage.setItem(&quot;kategori_subkategori_save_edit&quot;, datas);
                            console.log(&quot;datas kategori_subkategori_save_edit defined&quot;,datas);
                            jdata = JSON.parse(datas);
                            console.log(jdata);
                            console.log(&quot;jdata.length: &quot;+Object.keys(jdata).length);
                            if (Object.keys(jdata).length==1) {
                                if (jdata[0].ret == 1){
                                    alert(jdata[0].msg);
                                    //location.replace('http://&lt; ?=$HTTP_HOST;?>/mod_ec/form/as1031.php?former_kategori_subkategori=&lt; ?=$former_kategori_subkategori?>'+&quot;&amp;former_search_txt=&lt; ?=$former_search_txt?>&quot;+&quot;&amp;former_search_txt2=&lt; ?=$former_search_txt2?>&quot;+&quot;&amp;former_search_txt3=&lt; ?=$former_search_txt3?>&quot;);
                                    location.replace('http://172.28.108.46:5454/smile/mod_ec/form/as1031.php?former_kategori_subkategori=Y'+&quot;&amp;former_search_txt=&quot;+&quot;&amp;former_search_txt2=&quot;+&quot;&amp;sess=div-container-table&quot;+&quot;&amp;selected_data=&quot;+&quot;&amp;status_crud=sukses&quot;);
                                } else {
                                    alert(jdata[0].msg);
                                }
                            } else {
                                $(&quot;#saved_message&quot;).html(&quot;&quot;);
                                $(&quot;#saved_message&quot;).show();
                                var return_message = &quot;&quot;;
                                var failed = &quot;&quot;;
                                $.each(jdata, function(idx, val) {
                                    let data_ke = idx + 1;
                                    return_message = return_message + &quot;Data ke-&quot; + val.ke + &quot;: &quot; + val.msg + &quot;\n\r&lt;br/>&quot;;
                                    if (val.ret==&quot;1&quot;) {
                                        console.log(&quot;idx: &quot;+idx);
                                        $(&quot;#saved_message&quot;).append(&quot;&lt;span class='saved_message_success'>idx ke: &quot;+data_ke+&quot;, val ke: &quot;+val.ke+&quot;, val ret: &quot;+val.ret+&quot;, val msg: &quot;+val.msg+&quot;, val no urut: &quot;+val.noUrut+&quot;, val input: &quot;+val.input+&quot;, val status: &quot;+val.status+&quot;&lt;/span>&lt;br/>&lt;br/>                        \n\n\n\n&quot;);
                                        $(&quot;#saved_message&quot;).append(&quot;\n\r&quot;);
                                        $(&quot;#saved_message&quot;).append(&quot;\n\r&quot;);
                                    
                                        let div_color = &quot;&lt;div style='z-index:2;background: linear-gradient(159deg, rgba(173,223,173,1) 0%, rgba(50,205,50,1) 100%);opacity:0.3;position:absolute;border-radius:10px;left:0px;top:0px;right:0px;bottom:0px;'>&lt;/div>&quot;;
                                        $(&quot;.konten_form&quot;+val.ke).append(div_color);
                                        $(&quot;.konten_form&quot;+val.ke+&quot; input[type=text], .konten_form&quot;+val.ke+&quot; input[type=hidden]&quot;).attr('disabled','disabled').prop('disabled', 'disabled');
                                        $(&quot;.konten_form&quot;+val.ke+&quot; input[type=text], .konten_form&quot;+val.ke+&quot; input[type=hidden]&quot;).attr('readonly','readonly').prop('readonly', 'readonly');
                                        $(&quot;.konten_form&quot;+val.ke+&quot; select&quot;).select2(&quot;enable&quot;, false);
                                        console.log(&quot;data_ke: &quot;+data_ke);
                                    } else {
                                        $(&quot;#saved_message&quot;).append(&quot;&lt;span class='saved_message_failed'>idx ke: &quot;+data_ke+&quot;, val ke: &quot;+val.ke+&quot;, val ret: &quot;+val.ret+&quot;, val msg: &quot;+val.msg+&quot;, val no urut: &quot;+val.noUrut+&quot;, val input: &quot;+val.input+&quot;, val status: &quot;+val.status+&quot;&lt;/span>&lt;br/>&lt;br/>                        \n\n\n\n&quot;);
                                        $(&quot;#saved_message&quot;).append(&quot;\n\r&quot;);
                                        $(&quot;#saved_message&quot;).append(&quot;\n\r&quot;);
                                        //$(&quot;.konten_form&quot;+val.ke).append(&quot;&lt;input type='button' onclick='enableSelect2EditFailed(&quot;+val.ke+&quot;);' value='Enable Select2' />&quot;);
                                        failed += failed + &quot;Data ke-&quot; + val.ke + &quot;: &quot; + val.msg + &quot;\n\r&lt;br/>&quot;;
                                        
                                        $(&quot;.konten_form&quot;+val.ke+&quot; input[type=hidden]&quot;).removeAttr('disabled').prop('disabled', false);
                                        $(&quot;.konten_form&quot;+val.ke+&quot; input[type=hidden]&quot;).removeAttr('readonly').prop('readonly', false);
                                        $(&quot;.konten_form&quot;+val.ke+&quot; #subkategori&quot;+val.ke).removeAttr('disabled').prop('disabled', false);
                                        $(&quot;.konten_form&quot;+val.ke+&quot; #subkategori&quot;+val.ke).removeAttr('readonly').prop('readonly', false);
                                        //$(&quot;select&quot;).select2(&quot;enable&quot;, true);
                                        $(&quot;#kode_subkategori&quot;+val.ke).prop('disabled', false).removeAttr('disabled'); //hidden
                                        $(&quot;#kode_subkategori&quot;+val.ke).prop('readonly', false).removeAttr('readonly'); //hidden
                                    }
                                });
                                alert(return_message);
                                if (failed==&quot;&quot;) {
                                    //location.replace('http://&lt; ?=$HTTP_HOST;?>/mod_ec/form/as1031.php?former_kategori_subkategori=&lt; ?=$former_kategori_subkategori?>'+&quot;&amp;former_search_txt=&lt; ?=$former_search_txt?>&quot;+&quot;&amp;former_search_txt2=&lt; ?=$former_search_txt2?>&quot;+&quot;&amp;former_search_txt3=&lt; ?=$former_search_txt3?>&quot;);
                                    location.replace('http://172.28.108.46:5454/smile/mod_ec/form/as1031.php?former_kategori_subkategori=Y'+&quot;&amp;former_search_txt=&quot;+&quot;&amp;former_search_txt2=&quot;+&quot;&amp;sess=div-container-table&quot;+&quot;&amp;selected_data=&quot;+&quot;&amp;status_crud=sukses&quot;);
                                }
                            }
                            preloadMask(false);
                        },
                        complete: function(){
                            preloadMask(false);
                        },
                        error: function(){
                            alert(&quot;Terjadi kesalahan, coba beberapa saat lagi!&quot;);
                            preloadMask(false);
                        }
                    });
		}, function(){});
            }
	} 

	function kembali(){
            //window.location.replace('http://&lt; ?=$HTTP_HOST; ?>/mod_ec/form/as1031.php?former_kategori_subkategori=&lt; ?=$former_kategori_subkategori?>'+&quot;&amp;former_search_txt=&lt; ?=$former_search_txt?>&quot;+&quot;&amp;former_search_txt2=&lt; ?=$former_search_txt2?>&quot;+&quot;&amp;former_search_txt3=&lt; ?=$former_search_txt3?>&quot;);
            window.location.replace('http://172.28.108.46:5454/smile/mod_ec/form/as1031.php?former_kategori_subkategori=Y'+&quot;&amp;former_search_txt=&quot;+&quot;&amp;former_search_txt2=&quot;+&quot;&amp;sess=div-container-table&quot;+&quot;&amp;selected_data=&quot;);
	} 
		
	function emptyrows() {
            let html_data = &quot;&quot;;
            html_data += '&lt;tr class=&quot;nohover-color&quot;>';
            html_data += '&lt;td colspan=&quot;10&quot; style=&quot;text-align: center;&quot;>-- Data tidak ditemukan --&lt;/td>';
            html_data += '&lt;/tr>';
            $(&quot;#data_list&quot;).html(html_data);
	}

	function download() {
            const linkDownload  = '';
            window.open(linkDownload, '_blank').focus();
            //location.replace('http://&lt; ?= $HTTP_HOST; ?>/mod_ec/form/as1031.php?former_kategori_subkategori=&lt; ?=$former_kategori_subkategori?>'+&quot;&amp;former_search_txt=&lt; ?=$former_search_txt?>&quot;+&quot;&amp;former_search_txt2=&quot;+&quot;&amp;former_search_txt3=&lt; ?=$former_search_txt3?>&quot;);
            location.replace('http://172.28.108.46:5454/smile/mod_ec/form/as1031.php?former_kategori_subkategori=Y'+&quot;&amp;former_search_txt=&quot;+&quot;&amp;former_search_txt2=&quot;);
	}

	function regenerate(id_batch) {
            preloadMask(true);
            $.ajax({
                type: 'POST',
                url: &quot;http://172.28.108.46:5454/smile/mod_ec/ajax/as1031_action.php?&quot;+Math.random(),
                data: {
                    tipe: 'regenerate',
                    idBatch: id_batch
                },
                success: function(datas){
                    console.log(&quot;datas&quot;,datas);
                    jdata = JSON.parse(datas);
                    console.log(jdata);
                    if (jdata.ret == 1){
                        alert(jdata.msg);
                        //location.replace('http://&lt; ?=$HTTP_HOST; ?>/mod_ec/form/as1031.php?former_kategori_subkategori=&lt; ?=$former_kategori_subkategori?>'+&quot;&amp;former_search_txt=&lt; ?=$former_search_txt?>&quot;+&quot;&amp;former_search_txt2=&lt; ?=$former_search_txt2?>&quot;+&quot;&amp;former_search_txt3=&lt; ?=$former_search_txt3?>&quot;);
                        location.replace('http://172.28.108.46:5454/smile/mod_ec/form/as1031.php?former_kategori_subkategori=Y'+&quot;&amp;former_search_txt=&quot;+&quot;&amp;former_search_txt2=&quot;+&quot;&amp;sess=div-container-table&quot;+&quot;&amp;selected_data=&quot;+&quot;&amp;status_crud=sukses&quot;);
                    } else {
                        alert(jdata.msg);
                    }
                    preloadMask(false);
                },
                complete: function(){
                    preloadMask(false);
                },
                error: function(){
                    alert(&quot;Terjadi kesalahan, coba beberapa saat lagi!&quot;);
                    preloadMask(false);
                }
            });
	}
        
        function addInputs(kode_konten, mode, obj) { //T = KATEGORI, Y = SUB KATEGORI
            /*$('#konten_form').clone().insertAfter('div#konten2:last');*/
            //$(&quot;div[id^='konten_form']:last&quot;).after($('div#konten2').clone());
            //alert('test: '+kode_konten);
            
            //alert($(&quot;div#konten_form&quot;).length);
//            if (mode==&quot;change&quot;) {
//                //$(obj+&quot;.tipe_input,&quot;+obj+&quot;.kategori,&quot;+obj+&quot;.subkategori,&quot;+obj+&quot;.seq,&quot;+obj+&quot;.status&quot;).select2(&quot;destroy&quot;);
//                $(obj).remove();
//            }
            
            var no_konten_form = $(&quot;div#konten_form&quot;).length + 1;
            if (no_konten_form==11) {
                alert(&quot;Anda hanya bisa menambahkan 10 data.&quot;);
                return;
            }
            
            var no_urut = &quot;&quot;;
            for (let i = 1; i &lt;= 100; i++) {
                var selected = &quot;&quot;;
                if (no_konten_form==i) {
                    selected = &quot;selected='selected'&quot;;
                }
                no_urut += &quot;&lt;option value='&quot;+i+&quot;' &quot;+selected+&quot;>&quot;+i+&quot;&lt;/option>&quot;;
            }
            
            var input = '&lt;div id=&quot;konten_form&quot; class=&quot;konten_form'+no_konten_form+'&quot; style=&quot;float:left;position:relative;border-radius:10px;width:440px;height:128px;border:1px solid black;margin:10px;padding:40px 10px 10px 10px;&quot;>'+
                             '&lt;div style=&quot;z-index:2;color:white;background: -webkit-linear-gradient(left,#6ba5ff,#416fd6);position:absolute;border-top-left-radius:10px;border-top-right-radius:10px;padding:6px 2px 2px 2px;font-weight:bold;font-size:12px;height:20px;left:0px;top:0px;right:0px;text-align:center;&quot;>Data Ke-'+no_konten_form+'&lt;img style=&quot;float:right;width:15px;height:auto;margin-right:4px;cursor:pointer;&quot; src=&quot;../../images/removex.png&quot; onclick=&quot;$(\'.konten_form'+no_konten_form+'\').remove();&quot; />&lt;/div>';
            if (kode_konten==&quot;T&quot; || kode_konten==&quot;KATEGORI&quot;) { //kategori
                    input += '&lt;div class=&quot;form-row_kiri&quot;>'+
                                    '&lt;label style= &quot;text-align:right;width: 130px;&quot;>Tipe &lt;span style=&quot;color:#ff0000;&quot;>*&lt;/span> :&lt;/label>'+
                                    '&lt;select name=&quot;tipe_input[]&quot; class=&quot;tipe_input required&quot; style=&quot;width: 250px;&quot; accept=&quot;text/csv&quot; onchange=&quot;addInputs($(this).val(), \'change\', \'.konten_form'+no_konten_form+'\')&quot;>'+
                                            '&lt;option value=&quot;KATEGORI&quot; selected=&quot;selected&quot;>KATEGORI&lt;/option>'+
                                            //'&lt;option value=&quot;SUB KATEGORI&quot;>SUB KATEGORI&lt;/option>'+
                                    '&lt;/select>'+
                             '&lt;/div>'+
                             '&lt;div class=&quot;clear&quot;>&lt;/div>'+
                             '&lt;div class=&quot;form-row_kiri&quot;>'+
                                    '&lt;label style= &quot;text-align:right;width: 130px;&quot;>Nama Kategori &lt;span style=&quot;color:#ff0000;&quot;>*&lt;/span> :&lt;/label>'+
                                    '&lt;input type=&quot;text&quot; name=&quot;kategori[]&quot; class=&quot;kategori required&quot; style=&quot;width: 245px;&quot; accept=&quot;text/csv&quot; required oninvalid=&quot;this.setCustomValidity(\'Input nama kategori.\')&quot; oninput=&quot;this.setCustomValidity(\'\'); $(this).css(\'background-color\' , \'#FFFFFF\');&quot; />'+
                             '&lt;/div>'+
                             '&lt;div class=&quot;clear&quot;>&lt;/div>'+
                             '&lt;div class=&quot;form-row_kiri&quot;>'+
                                    '&lt;label style= &quot;text-align:right;width: 130px;&quot;>No Urut &lt;span style=&quot;color:#ff0000;&quot;>*&lt;/span> :&lt;/label>'+
                                    //'&lt;select name=&quot;kategori[]&quot; id=&quot;kategori0&quot; class=&quot;kategori required&quot; style=&quot;width: 250px;display:none;&quot; accept=&quot;text/csv&quot;>'+
                                    //'&lt;/select>'+
                                    '&lt;select name=&quot;seq[]&quot; class=&quot;seq required&quot; style=&quot;width: 250px;&quot; accept=&quot;text/csv&quot;>'+
                                        no_urut+
                                    '&lt;/select>'+
                             '&lt;/div>'+
                             '&lt;div class=&quot;clear&quot;>&lt;/div>'+
                             '&lt;div class=&quot;form-row_kiri&quot;>'+
                                    '&lt;label style= &quot;text-align:right;width: 130px;&quot;>Status &lt;span style=&quot;color:#ff0000;&quot;>*&lt;/span> :&lt;/label>'+
                                    '&lt;select name=&quot;status[]&quot; class=&quot;status required&quot; style=&quot;width: 250px;&quot; accept=&quot;text/csv&quot;>'+
                                            '&lt;option value=&quot;N&quot; selected=&quot;selected&quot;>TAMBAH DATA KATEGORI&lt;/option>'+
                                            '&lt;option value=&quot;Y&quot; disabled=&quot;disabled&quot;>AKTIF&lt;/option>'+
                                            '&lt;option value=&quot;T&quot; disabled=&quot;disabled&quot;>NON AKTIF&lt;/option>'+
                                            '&lt;option value=&quot;SF0&quot; disabled=&quot;disabled&quot;>PENYESUAIAN NAMA KATEGORI&lt;/option>'+
                                            '&lt;option value=&quot;SF1&quot; disabled=&quot;disabled&quot;>PENYESUAIAN NOMOR URUT&lt;/option>'+
                                    '&lt;/select>'+
                             '&lt;/div>'+
                        '&lt;/div>';
                $(&quot;#konten2&quot;).append(input);
                $('select').select2();
                //$('select.tipe_input, select.kategori, select.seq, select.status').select2();
                $('select.tipe_input, select.seq, select.status').select2();
            } else if (kode_konten==&quot;Y&quot; || kode_konten==&quot;SUB KATEGORI&quot;) { //sub kategori
                /*var no_urut2 = &quot;&quot;;
                for (let i = 1; i &lt;= 100; i++) {
                    no_urut2 += &quot;&lt;option value='&quot;+i+&quot;'>KATEGORI &quot;+i+&quot;&lt;/option>&quot;;
                }
                //no_urut2 = $('#konten_form.kategori:first').html();*/
                //alert(no_urut2);
                
                    input += '&lt;div class=&quot;form-row_kiri&quot;>'+
                                    '&lt;label style= &quot;text-align:right;width: 130px;&quot;>Tipe &lt;span style=&quot;color:#ff0000;&quot;>*&lt;/span> :&lt;/label>'+
                                    '&lt;select name=&quot;tipe_input[]&quot; class=&quot;tipe_input required&quot; style=&quot;width: 250px;&quot; accept=&quot;text/csv&quot; onchange=&quot;addInputs($(this).val(), \'change\', \'.konten_form'+no_konten_form+'\')&quot;>'+
                                            //'&lt;option value=&quot;KATEGORI&quot;>KATEGORI&lt;/option>'+
                                            '&lt;option value=&quot;SUB KATEGORI&quot; selected=&quot;selected&quot;>SUB KATEGORI&lt;/option>'+
                                    '&lt;/select>'+
                             '&lt;/div>'+
                             '&lt;div class=&quot;clear&quot;>&lt;/div>'+
                             '&lt;div class=&quot;form-row_kiri&quot;>'+
                                    '&lt;label style= &quot;text-align:right;width: 130px;&quot;>Nama Kategori &lt;span style=&quot;color:#ff0000;&quot;>*&lt;/span> :&lt;/label>'+
                                    //'&lt;select name=&quot;kategori[]&quot; class=&quot;kategori&quot; style=&quot;width: 250px;&quot; accept=&quot;text/csv&quot;>&lt;/select>'+
                                    '&lt;div id=&quot;tool_placeholder_'+no_konten_form+'&quot;>&lt;/div>'+
                             '&lt;/div>'+
                             '&lt;div class=&quot;clear&quot;>&lt;/div>'+
                             '&lt;div class=&quot;form-row_kiri&quot;>'+
                                    '&lt;label style= &quot;text-align:right;width: 130px;&quot;>Nama Sub Kategori &lt;span style=&quot;color:#ff0000;&quot;>*&lt;/span> :&lt;/label>'+
                                    '&lt;input type=&quot;text&quot; name=&quot;subkategori[]&quot; class=&quot;subkategori required&quot; style=&quot;width: 245px;&quot; accept=&quot;text/csv&quot; required oninvalid=&quot;this.setCustomValidity(\'Input nama sub kategori.\')&quot; oninput=&quot;this.setCustomValidity(\'\'); $(this).css(\'background-color\' , \'#FFFFFF\');&quot; />'+
                             '&lt;/div>'+
                             '&lt;div class=&quot;clear&quot;>&lt;/div>'+
                             '&lt;div class=&quot;form-row_kiri&quot;>'+
                                    '&lt;label style= &quot;text-align:right;width: 130px;&quot;>No Urut &lt;span style=&quot;color:#ff0000;&quot;>*&lt;/span> :&lt;/label>'+
                                    '&lt;select name=&quot;seq[]&quot; class=&quot;seq required&quot; style=&quot;width: 250px;&quot; accept=&quot;text/csv&quot;>'+
                                        no_urut+
                                    '&lt;/select>'+
                             '&lt;/div>'+
                             '&lt;div class=&quot;clear&quot;>&lt;/div>'+
                             '&lt;div class=&quot;form-row_kiri&quot;>'+
                                    '&lt;label style= &quot;text-align:right;width: 130px;&quot;>Status &lt;span style=&quot;color:#ff0000;&quot;>*&lt;/span> :&lt;/label>'+
                                    '&lt;select name=&quot;status[]&quot; class=&quot;status required&quot; style=&quot;width: 250px;&quot; accept=&quot;text/csv&quot;>'+
                                            '&lt;option value=&quot;N&quot; selected=&quot;selected&quot;>TAMBAH DATA SUB KATEGORI&lt;/option>'+
                                            '&lt;option value=&quot;Y&quot; disabled=&quot;disabled&quot;>AKTIF&lt;/option>'+
                                            '&lt;option value=&quot;T&quot; disabled=&quot;disabled&quot;>NON AKTIF&lt;/option>'+
                                            '&lt;option value=&quot;SF0&quot; disabled=&quot;disabled&quot;>PENYESUAIAN NAMA KATEGORI&lt;/option>'+
                                            '&lt;option value=&quot;SF0&quot; disabled=&quot;disabled&quot;>PENYESUAIAN NAMA SUB KATEGORI&lt;/option>'+
                                            '&lt;option value=&quot;SF1&quot; disabled=&quot;disabled&quot;>PENYESUAIAN NOMOR URUT&lt;/option>'+
                                    '&lt;/select>'+
                             '&lt;/div>'+
                        '&lt;/div>';
                $(&quot;#konten2&quot;).append(input);
                
                //$('select.kategori').select2(&quot;destroy&quot;);
                var noOfSelect2 = $('.kategori').length;
                //var clonedSelect2 = $('.kategori').first().clone(true);
                var clonedSelect2 = $('#kategori0').clone(true);
                clonedSelect2.insertBefore(&quot;#tool_placeholder_&quot;+no_konten_form);
                clonedSelect2.attr('id', 'kategori'+noOfSelect2);
                clonedSelect2.attr('name', 'kategori[]');
                clonedSelect2.attr('class', 'kategori');
                clonedSelect2.attr('style', 'width: 250px;');
                clonedSelect2.attr('accept', 'text/csv');
                $('select').select2();
                //$('select.kategori').select2();
                $('select#kategori'+noOfSelect2).select2();
                $('select.tipe_input, select.kategori, select.seq, select.status').select2();
            }
            
            $('.kategori').keypress(function (e) {
                console.log(&quot;keypress button: e.key=&quot;+e.key+&quot; - e.code=&quot;+e.code+&quot; - e.which=&quot;+e.which);
                /*if (e.which === 44 || e.key === ',' || e.key === '&lt;' || e.key === '.' || e.key === '>' || e.key === '?' ||
                    e.key === '/' || e.key === ':' || e.key === ';' || e.key === '&quot;' || e.key === &quot;'&quot; || e.key === '{' ||
                    e.key === '[' || e.key === '}' || e.key === ']' || e.key === '|' || e.key === '\\' || e.key === '`' ||
                    e.key === '~' || e.key === '!' || e.key === '@' || e.key === '#' || e.key === '$' || e.key === '%' ||
                    e.key === '^' || e.key === '&amp;' || e.key === '*' || e.key === '(' || e.key === ')' || e.key === '_' ||
                    e.key === '-' || e.key === '+' || e.key === '=') {*/
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:'&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                    //alert(&quot;comma detected1&quot;);
                /*} else {
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replaceAll(/[`~!@#$%^&amp;*()_|+\=?;:'&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                }*/
            }).keyup(function (e) {
                /*if (e.which === 44 || e.key === ',' || e.key === '&lt;' || e.key === '.' || e.key === '>' || e.key === '?' ||
                    e.key === '/' || e.key === ':' || e.key === ';' || e.key === '&quot;' || e.key === &quot;'&quot; || e.key === '{' ||
                    e.key === '[' || e.key === '}' || e.key === ']' || e.key === '|' || e.key === '\\' || e.key === '`' ||
                    e.key === '~' || e.key === '!' || e.key === '@' || e.key === '#' || e.key === '$' || e.key === '%' ||
                    e.key === '^' || e.key === '&amp;' || e.key === '*' || e.key === '(' || e.key === ')' || e.key === '_' ||
                    e.key === '-' || e.key === '+' || e.key === '=') {*/
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:'&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                    //alert(&quot;comma detected1&quot;);
                /*} else {
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replaceAll(/[`~!@#$%^&amp;*()_|+\=?;:'&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                }*/
            });
            
            $('.subkategori').keypress(function (e) {
                console.log(&quot;keypress button: e.key=&quot;+e.key+&quot; - e.code=&quot;+e.code+&quot; - e.which=&quot;+e.which);
                /*if (e.which === 44 || e.key === ',' || e.key === '&lt;' || e.key === '.' || e.key === '>' || e.key === '?' ||
                    e.key === '/' || e.key === ':' || e.key === ';' || e.key === '&quot;' || e.key === &quot;'&quot; || e.key === '{' ||
                    e.key === '[' || e.key === '}' || e.key === ']' || e.key === '|' || e.key === '\\' || e.key === '`' ||
                    e.key === '~' || e.key === '!' || e.key === '@' || e.key === '#' || e.key === '$' || e.key === '%' ||
                    e.key === '^' || e.key === '&amp;' || e.key === '*' || e.key === '(' || e.key === ')' || e.key === '_' ||
                    e.key === '-' || e.key === '+' || e.key === '=') {*/
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:'&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                    //alert(&quot;comma detected1&quot;);
                /*} else {
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:'&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                }*/
            }).keyup(function (e) {
                /*if (e.which === 44 || e.key === ',' || e.key === '&lt;' || e.key === '.' || e.key === '>' || e.key === '?' ||
                    e.key === '/' || e.key === ':' || e.key === ';' || e.key === '&quot;' || e.key === &quot;'&quot; || e.key === '{' ||
                    e.key === '[' || e.key === '}' || e.key === ']' || e.key === '|' || e.key === '\\' || e.key === '`' ||
                    e.key === '~' || e.key === '!' || e.key === '@' || e.key === '#' || e.key === '$' || e.key === '%' ||
                    e.key === '^' || e.key === '&amp;' || e.key === '*' || e.key === '(' || e.key === ')' || e.key === '_' ||
                    e.key === '-' || e.key === '+' || e.key === '=') {*/
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:'&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                    //alert(&quot;comma detected1&quot;);
                /*} else {
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:'&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                }*/
            });
        }


	.div-container{
		min-width: 700px;
		width: 100%;
	}
	.div-header{
		min-width: 700px;
	}
	.div-body{
		overflow-x: auto; 
		overflow-y: auto; 
		white-space: nowrap;
	}
	.div-data{
		overflow-x: auto; 
		overflow-y: auto; 
		white-space: nowrap;
	}
	.div-footer{
		padding-top: 10px;
		border-bottom: 1px solid #eeeeee;
	}
	.hr-double{
		border-top:3px double #8c8c8c;
		border-bottom:3px double #8c8c8c;
	}
  .hr-double-top{
    border-top:3px double #8c8c8c;
	}
  .hr-double-bottom{
  	border-bottom:3px double #8c8c8c;
	}
	.hr-double-left{
    border-left:3px double #8c8c8c;
	}
  .hr-double-right{
    border-right:3px double #8c8c8c;
	}
	.table-data{
		width: 100%;
		border-collapse: collapse;
		border-color: #c0c0c0;
		background-color: #ffffff;
	}
	.table-data th{
		padding: 10px 6px 10px 6px;
		font-weight: bold;
		text-align: left;
	}
	.table-data td{
		padding: 4px 6px 4px 6px;
		text-align: left;
		border-bottom: 1px solid #c0c0c0;
	}
	.table-data tr:last-child td{
		border-bottom:3px double #8c8c8c;
	}
	.table-data tbody tr:hover{
		cursor: pointer;
		background-color:#f5f5f5;
	}
  .nohover-color:hover {
		cursor: pointer!important;
    background-color:#FFFFFF!important;
	}
	.value-modified{
    background-color: #b4eeb4!important;
  }
  .l_frm{width: 180px; clear: left; float: left;margin-bottom: 2px;text-align: right; margin-right: 2px;}
    .r_frm{float: left;margin-bottom: 2px;}
    .r_frm input,.r_frm select {
        border-radius: 2px; 
        -moz-border-radius: 2px; 
        -webkit-border-radius: 2px; 
        border: 1px solid #bbb;
    }
    .column {
	  float: left;
	  padding: 1px;
	  /*height: 200px; /* Should be removed. Only for demonstration */
	}

	/* Clear floats after the columns */
	.row:after {
	  content: &quot;&quot;;
	  display: table;
	  clear: both;
	}

	input.button-accept {
	  width: 100%;
	  height:30px;
	  box-sizing: border-box;
	  border: 2px solid #ccc;
	  border-radius: 4px;
	  font-size: 18px;
	  font-weight: bold;
	  color:white;
	  background-color: #0091FF	;
	  /*background-image: url(../../images/app_form_edit.png);*/ 
	  background-image: url(../../images/accept.png);
	  background-position: 0px 0px; 
	  background-repeat: no-repeat;
	  padding: 0px 20px 12px 40px;
	}

	input.button-close {
	  width: 100%;
	  height:30px;
	  box-sizing: border-box;
	  border: 2px solid #ccc;
	  border-radius: 4px;
	  font-size: 18px;
	  font-weight: bold;
	  color:white;
	  background-color: #0091FF	;
	  background-image: url(../../images/cancel.png);
	  /*background-size: 20px;*/
	  background-position: 0px 0px; 
	  background-repeat: no-repeat;
	  padding: 0px 20px 12px 40px;
	}

	input.button-proses {
	  width: 100%;
	  height:30px;
	  box-sizing: border-box;
	  border: 2px solid #ccc;
	  border-radius: 4px;
	  font-size: 18px;
	  font-weight: bold;
	  color:white;
	  background-color: #0091FF	;
	  background-image: url(../../images/proses.png);
	  background-position: 0px 0px; 
	  background-repeat: no-repeat;
	  padding: 0px 20px 12px 40px;
	}

	input.button-verif {
	  width: 100%;
	  height:30px;
	  box-sizing: border-box;
	  border: 2px solid #ccc;
	  border-radius: 4px;
	  font-size: 18px;
	  font-weight: bold;
	  color:white;
	  background-color: #0091FF	;
	  background-image: url(../../images/open.gif);
	  background-position: 0px 0px; 
	  background-repeat: no-repeat;
	  padding: 0px 20px 12px 40px;
	}

	input.button-back {
	  width: 100%;
	  height:30px;
	  box-sizing: border-box;
	  border: 2px solid #ccc;
	  border-radius: 4px;
	  font-size: 18px;
	  font-weight: bold;
	  color:white;
	  background-color: #0091FF	;
	  background-image: url(../../images/application_view_columns.png); 
	  background-position: 0px 0px; 
	  background-repeat: no-repeat;
	  padding: 0px 20px 12px 40px;
	}




	AS1031 - CMS FAQ Kategori dan Sub Kategori [new] 


    
    
        
            
                
                
                
                    
                    
                    	
                        
                            
                                
                                    
                                    
                                        
                                            
                                            Detil Data					
                                             
                                             
                                             
                                            
                                                
                                                
                                                                                                     
                                                         Data Ke-1
                                                                                                                      
                                                                    Tipe * :
                                                                    
                                                                            
                                                                            SUB KATEGORI
                                                                    SUB KATEGORI
                                                              
                                                             
                                                             
                                                                    Nama Kategori * :
                                                                    
                                                                        FKAT001 - UMUMFKAT002 - PENERIMA UPAHFKAT003 - BUKAN PENERIMA UPAHFKAT004 - JAMINAN HARI TUA (JHT)FKAT005 - JAMINAN KECELAKAAN KERJA (JKK)FKAT006 - JAMINAN KEMATIAN (JKM)FKAT007 - JAMINAN PENSIUN (JP)FKAT008 - JAMINAN KEHILANGAN PEKERJAAN (JKP)FKAT009 - JASA KONSTRUKSIFKAT010 - LAYANAN SYARIAHFKAT011 - MANFAAT LAYANAN TAMBAHAN (MLT)FKAT012 - KANAL LAYANANFKAT013 - E-CHANNELFKAT014 - PEKERJA MIGRAN INDONESIA (PMI)FKAT015 - UJI KATALON 202412                                                                    FKAT015 - UJI KATALON 202412
                                                             		
                                                             
                                                             
                                                                    Nama Sub Kategori * :
                                                                    
                                                             		
                                                              
                                                             
                                                                    No Urut * :
                                                                    
                                                                            123456789101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899100                                                                    1
                                                              
                                                              
                                                             
                                                                    Status * :
                                                                    
                                                                            TAMBAH DATA SUB KATEGORI
                                                                            
                                                                    TAMBAH DATA SUB KATEGORI
                                                             
                                                                                                              
                                                                                                
                                            
                                        
                                    	
                                    
                                    
                                    
                                    
                                    
                                    
                                             

                                                                                		
                                            											
                                                    
                                                    
                                                    
                                            
                                     	
                                    
                                    
                                    
                                            Keterangan:
                                             Klik tombol TUTUP untuk kembali ke halaman utama
                                    
                                    
                                							
                            		
                        
                        
                        
                    
                	
            
        
    



 

  
    Loading...
  


/html[1]</value>
      <webElementGuid>6c434ce9-e1bf-4962-b4c7-a75c05c2131f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]</value>
      <webElementGuid>5f9e5f1d-fe50-456c-a90c-440882a92e48</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>ref_element</name>
      <type>Main</type>
      <value>Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/iframe_AS1031-CMS FAQ Kategori dan Sub Kate_6caf2b</value>
      <webElementGuid>e26d506b-2dd1-47c7-b59d-b37e0bf4fe19</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>//*/text()[normalize-space(.)='']/parent::*</value>
      <webElementGuid>26f1c3cc-940a-4d1a-8aa9-550c69803d22</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//html</value>
      <webElementGuid>dc554c10-5b34-41a2-9a7c-95bb14376cfb</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//html[(text() = concat(&quot;
  
   - 
  
  
	
    

$(document).ready(function(){	
	setTimeout(function(){
		$(&quot; , &quot;'&quot; , &quot;#loading&quot; , &quot;'&quot; , &quot;).hide();
		$(&quot; , &quot;'&quot; , &quot;#loading-mask&quot; , &quot;'&quot; , &quot;).hide();
	}, 250);
});
function alertError(msg){
	window.parent.Ext.MessageBox.show({
	   title: &quot; , &quot;'&quot; , &quot;Perhatian&quot; , &quot;'&quot; , &quot;,
	   msg: msg,
	   buttons: window.parent.Ext.MessageBox.OK,
	   icon: &quot; , &quot;'&quot; , &quot;x-message-box-error&quot; , &quot;'&quot; , &quot;
   });	
}
function alert(msg){
	window.parent.Ext.MessageBox.alert(&quot; , &quot;'&quot; , &quot;Perhatian&quot; , &quot;'&quot; , &quot;, msg);
}
function alertMsg(msg){
	window.parent.Ext.MessageBox.alert(&quot; , &quot;'&quot; , &quot;Informasi&quot; , &quot;'&quot; , &quot;, msg);
}
window.notificationCountAlertNew = window.notificationCountAlertNew || 0;
function showAutoCloseAlert(msg, typeAlert = &quot; , &quot;'&quot; , &quot;info&quot; , &quot;'&quot; , &quot;, title = &quot;Perhatian&quot;, countClose = 4000) {
	console.log(window.notificationCountAlertNew);
	let baseColorAlert = &quot; , &quot;'&quot; , &quot;#4CAF50&quot; , &quot;'&quot; , &quot;
	let backGroundColorAlert = &quot; , &quot;'&quot; , &quot;#f0f8ff&quot; , &quot;'&quot; , &quot;
	let textColroAlert = &quot; , &quot;'&quot; , &quot;#000&quot; , &quot;'&quot; , &quot;
	switch (typeAlert) {
		case &quot; , &quot;'&quot; , &quot;info&quot; , &quot;'&quot; , &quot;:
			baseColorAlert = &quot; , &quot;'&quot; , &quot;#3A87AD&quot; , &quot;'&quot; , &quot;
			backGroundColorAlert = &quot; , &quot;'&quot; , &quot;#f0f8ff&quot; , &quot;'&quot; , &quot;
			textColroAlert = &quot; , &quot;'&quot; , &quot;#000&quot; , &quot;'&quot; , &quot;
			break;
		case &quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;:
			baseColorAlert = &quot; , &quot;'&quot; , &quot;#4CAF50&quot; , &quot;'&quot; , &quot;
			backGroundColorAlert = &quot; , &quot;'&quot; , &quot;#f0f8ff&quot; , &quot;'&quot; , &quot;
			textColroAlert = &quot; , &quot;'&quot; , &quot;#000&quot; , &quot;'&quot; , &quot;
			break;
		case &quot; , &quot;'&quot; , &quot;warning&quot; , &quot;'&quot; , &quot;:
			baseColorAlert = &quot; , &quot;'&quot; , &quot;#C09853&quot; , &quot;'&quot; , &quot;
			backGroundColorAlert = &quot; , &quot;'&quot; , &quot;#f0f8ff&quot; , &quot;'&quot; , &quot;
			textColroAlert = &quot; , &quot;'&quot; , &quot;#000&quot; , &quot;'&quot; , &quot;
			break;
		case &quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;:
			baseColorAlert = &quot; , &quot;'&quot; , &quot;#B94A48&quot; , &quot;'&quot; , &quot;
			backGroundColorAlert = &quot; , &quot;'&quot; , &quot;#f0f8ff&quot; , &quot;'&quot; , &quot;
			textColroAlert = &quot; , &quot;'&quot; , &quot;#000&quot; , &quot;'&quot; , &quot;
			break;
		default:
			baseColorAlert = &quot; , &quot;'&quot; , &quot;#4CAF50&quot; , &quot;'&quot; , &quot;
			backGroundColorAlert = &quot; , &quot;'&quot; , &quot;#f0f8ff&quot; , &quot;'&quot; , &quot;
			textColroAlert = &quot; , &quot;'&quot; , &quot;#000&quot; , &quot;'&quot; , &quot;
			break;
	}
	var baseY = 20;
	var notificationHeight = 110;
	var currentY = baseY + (window.notificationCountAlertNew * notificationHeight);
	if (window.notificationCountAlertNew+1 &lt; 6) {
		var win = window.parent.Ext.create(&quot; , &quot;'&quot; , &quot;Ext.window.Window&quot; , &quot;'&quot; , &quot;, {
			title: title,
			html: msg,
			width: 450,
			height: 100,
			closable: true,
			bodyStyle: `padding: 10px; background-color: ${backGroundColorAlert}; color: #000; border-radius: 8px;`,
			autoShow: true,
			modal: false,
			shadow: true,
			resizable: false,
			draggable: false,
			x: (window.parent.Ext.getBody().getViewSize().width - 450) / 2,
			y: currentY,
			style: {
				borderColor: `${baseColorAlert} !important`,
				borderStyle: &quot; , &quot;'&quot; , &quot;solid&quot; , &quot;'&quot; , &quot;,
				borderWidth: &quot; , &quot;'&quot; , &quot;2px&quot; , &quot;'&quot; , &quot;,
				borderRadius: &quot; , &quot;'&quot; , &quot;8px&quot; , &quot;'&quot; , &quot;
			},
			header: {
				style: {
					borderColor: `${baseColorAlert} !important`,
					backgroundColor: `${baseColorAlert}`,
					color: &quot; , &quot;'&quot; , &quot;#fff&quot; , &quot;'&quot; , &quot;,
					borderRadius: &quot; , &quot;'&quot; , &quot;8px 8px 0 0&quot; , &quot;'&quot; , &quot;
				},
			},
			listeners: {
				afterrender: function(win) {
					// Remove the background color of the existing close button
					var closeButton = win.header.el.down(&quot; , &quot;'&quot; , &quot;.x-tool-close&quot; , &quot;'&quot; , &quot;);
					if (closeButton) {
						closeButton.setStyle({
							backgroundColor: &quot; , &quot;'&quot; , &quot;transparent&quot; , &quot;'&quot; , &quot;, // Remove background color
							border: &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;, // Remove border if any
							color: &quot; , &quot;'&quot; , &quot;#fff&quot; , &quot;'&quot; , &quot; // Set icon color
						});
					}

					// Close the window after a custom delay
					window.parent.Ext.defer(function() {
						win.close();
						if (window.notificationCountAlertNew > 0) {
							window.notificationCountAlertNew--; // Decrement the counter only if it&quot; , &quot;'&quot; , &quot;s greater than 0
						}
					}, countClose); // Time in milliseconds (4 seconds)
				},
				close: function(win) {
					// Trigger when the X button is clicked
					if (window.notificationCountAlertNew > 0) {
						window.notificationCountAlertNew--; // Decrement the counter only if it&quot; , &quot;'&quot; , &quot;s greater than 0
					}
				}
			}
		});

		window.notificationCountAlertNew++;
	}
}

#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}






    .saved_message_success {
        font-weight:bold;font-size:12px;color:black;padding-bottom:30px;padding-top:30px;
    }
    .saved_message_failed {
        font-weight:bold;font-size:12px;color:red;padding-bottom:30px;padding-top:30px;
    }

    div.DialogMask
    {
        padding: 10px;
        position: fixed;
        top: 0;
        right: 0;
        bottom: 0;
        left: 0;
        z-index: 50;
        background-color: #606060;
        filter: progid:DXImageTransform.Microsoft.Alpha(Opacity=50);
        -moz-opacity: .5;
        opacity: .5;
    }





  



    //logger.disableLogger();
    logger.enableLogger();
	$(document).ready(function(){
            $(&quot;input[type=text]&quot;).keyup(function(){
                    //$(this).val($(this).val().toUpperCase());
            });
            
            $(&quot; , &quot;'&quot; , &quot;.kategori&quot; , &quot;'&quot; , &quot;).keypress(function (e) {
                console.log(&quot;keypress button: e.key=&quot;+e.key+&quot; - e.code=&quot;+e.code+&quot; - e.which=&quot;+e.which);
                /*if (e.which === 44 || e.key === &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&lt;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;>&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;?&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&quot;&quot; , &quot;'&quot; , &quot; || e.key === &quot;&quot; , &quot;'&quot; , &quot;&quot; || e.key === &quot; , &quot;'&quot; , &quot;{&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;[&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;}&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;|&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;\\&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;`&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;~&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;!&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;@&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;$&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;%&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;^&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;*&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;(&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;)&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;_&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;+&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;=&quot; , &quot;'&quot; , &quot;) {*/
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                    //alert(&quot;comma detected1&quot;);
                /*} else {
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replaceAll(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                }*/
            }).keyup(function (e) {
                /*if (e.which === 44 || e.key === &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&lt;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;>&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;?&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&quot;&quot; , &quot;'&quot; , &quot; || e.key === &quot;&quot; , &quot;'&quot; , &quot;&quot; || e.key === &quot; , &quot;'&quot; , &quot;{&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;[&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;}&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;|&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;\\&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;`&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;~&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;!&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;@&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;$&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;%&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;^&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;*&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;(&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;)&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;_&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;+&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;=&quot; , &quot;'&quot; , &quot;) {*/
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                    //alert(&quot;comma detected1&quot;);
                /*} else {
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replaceAll(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                }*/
            });
            
            $(&quot; , &quot;'&quot; , &quot;.subkategori&quot; , &quot;'&quot; , &quot;).keypress(function (e) {
                console.log(&quot;keypress button: e.key=&quot;+e.key+&quot; - e.code=&quot;+e.code+&quot; - e.which=&quot;+e.which);
                /*if (e.which === 44 || e.key === &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&lt;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;>&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;?&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&quot;&quot; , &quot;'&quot; , &quot; || e.key === &quot;&quot; , &quot;'&quot; , &quot;&quot; || e.key === &quot; , &quot;'&quot; , &quot;{&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;[&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;}&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;|&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;\\&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;`&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;~&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;!&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;@&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;$&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;%&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;^&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;*&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;(&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;)&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;_&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;+&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;=&quot; , &quot;'&quot; , &quot;) {*/
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                    //alert(&quot;comma detected1&quot;);
                /*} else {
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                }*/
            }).keyup(function (e) {
                /*if (e.which === 44 || e.key === &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&lt;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;>&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;?&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&quot;&quot; , &quot;'&quot; , &quot; || e.key === &quot;&quot; , &quot;'&quot; , &quot;&quot; || e.key === &quot; , &quot;'&quot; , &quot;{&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;[&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;}&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;|&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;\\&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;`&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;~&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;!&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;@&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;$&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;%&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;^&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;*&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;(&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;)&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;_&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;+&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;=&quot; , &quot;'&quot; , &quot;) {*/
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                    //alert(&quot;comma detected1&quot;);
                /*} else {
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                }*/
            });

            $(&quot; , &quot;'&quot; , &quot;select.tipe_input&quot; , &quot;'&quot; , &quot;).select2();
            $(&quot; , &quot;'&quot; , &quot;select#kategori&quot; , &quot;'&quot; , &quot;).select2();
            $(&quot; , &quot;'&quot; , &quot;select.kategori&quot; , &quot;'&quot; , &quot;).select2();
            $(&quot; , &quot;'&quot; , &quot;select#tipe&quot; , &quot;'&quot; , &quot;).select2();
            $(&quot; , &quot;'&quot; , &quot;select.tipe&quot; , &quot;'&quot; , &quot;).select2();
            $(&quot; , &quot;'&quot; , &quot;select.seq&quot; , &quot;'&quot; , &quot;).select2();
            $(&quot; , &quot;'&quot; , &quot;select.status&quot; , &quot;'&quot; , &quot;).select2();
            $(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;).select2();

            $(window).bind(&quot;resize&quot;, function(){
                resize();
            });
            resize();
            // filter();
            /** list checkbox npp */
            window.list_npp = [];
	});
	var asyncPreloadStart;

	function asyncPreloadX(state){
            if (state == true) {
                asyncPreloadStart = setInterval(function() {
                    $(&quot; , &quot;'&quot; , &quot;#loading&quot; , &quot;'&quot; , &quot;).show();
                    $(&quot; , &quot;'&quot; , &quot;#loading-mask&quot; , &quot;'&quot; , &quot;).show();
                }, 50);
            } else {
                $(&quot; , &quot;'&quot; , &quot;#loading&quot; , &quot;'&quot; , &quot;).hide();
                $(&quot; , &quot;'&quot; , &quot;#loading-mask&quot; , &quot;'&quot; , &quot;).hide();
                clearInterval(asyncPreloadStart);
            }
	}
        
        function preloadMask(param_true_false) {
            if (param_true_false==true) {
                if ($(&quot; , &quot;'&quot; , &quot;.DialogMask&quot; , &quot;'&quot; , &quot;).length == 0) {
                    $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;DialogMask&quot; style=&quot;z-index:20000;background-color:black;opacity:0.5;width:100%;height:100vh;display:flex;justify-content:center;align-items:center;&quot;>&quot; , &quot;'&quot; , &quot;+
                                        &quot; , &quot;'&quot; , &quot;&lt;img style=&quot;width:40px;height:auto;&quot; src=&quot;http://172.28.108.46:5454/smile/images/loading.gif&quot; />&quot; , &quot;'&quot; , &quot;+
                                        &quot; , &quot;'&quot; , &quot;&lt;span style=&quot;display:inline-block;position:fixed;bottom:130px;right:0px;left:0px;text-align:center;color:white;font-weight:bold;font-size:12px;font-style:italic;padding-left:10px;&quot;>in progress...&lt;/span>&quot; , &quot;'&quot; , &quot;+
                                     &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;);
                }
                $(&quot; , &quot;'&quot; , &quot;.DialogMask&quot; , &quot;'&quot; , &quot;).show();
            } else {
                $(&quot; , &quot;'&quot; , &quot;.DialogMask&quot; , &quot;'&quot; , &quot;).hide();
            }
        }
	
	function getValue(val){
            return val == null || val == undefined ? &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; : val;
	}

	function search_by_changed(){
            $(&quot;#search_txt&quot;).val(&quot;&quot;);
            $(&quot;#search_txt2&quot;).val(&quot;&quot;);
	}

	function isNumber(evt) {
            evt = (evt) ? evt : window.event;
            var charCode = (evt.which) ? evt.which : evt.keyCode;
            if (charCode > 31 &amp;&amp; (charCode &lt; 48 || charCode > 57)) {
                return false;
            }
            return true;
	}
	
	function validateDigit(evt) {
            var theEvent = evt || window.event;
            var key = theEvent.keyCode || theEvent.which;
            key = String.fromCharCode(key);
            var regex = /[0-9]|\./;
            if (!regex.test(key)) {
                theEvent.returnValue = false;
                if (theEvent.preventDefault)
                    theEvent.preventDefault();
            }
        }
	
	function Comma(Num) { //function to add commas to textboxes
            Num += &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            Num = Num.replace(&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            Num = Num.replace(&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            Num = Num.replace(&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            Num = Num.replace(&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            Num = Num.replace(&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            Num = Num.replace(&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            x = Num.split(&quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;);
            x1 = x[0];
            x2 = x.length > 1 ? &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot; + x[1] : &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            var rgx = /(\d+)(\d{3})/;
            while (rgx.test(x1))
                            x1 = x1.replace(rgx, &quot; , &quot;'&quot; , &quot;$1&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;$2&quot; , &quot;'&quot; , &quot;);
            return x1 + x2;
	}

	function isNumberKey(evt) {
            var theEvent = evt || window.event;
            var key = theEvent.keyCode || theEvent.which;
            key = String.fromCharCode(key);
            if (key.length == 0)
                    return;
            var regex = /^[0-9\b]+$/;
            if (!regex.test(key)) {
                    theEvent.returnValue = false;
                    if (theEvent.preventDefault)
                            theEvent.preventDefault();
            }
	}

	function resize(){
            $(&quot;#div_container&quot;).width($(&quot;#div_dummy&quot;).width());

            $(&quot;#div_header&quot;).width($(&quot;#div_dummy&quot;).width());
            $(&quot;#div_body&quot;).width($(&quot;#div_dummy&quot;).width());
            $(&quot;#div_footer&quot;).width($(&quot;#div_dummy&quot;).width());

            $(&quot;#div_filter&quot;).width(0);
            $(&quot;#div_data&quot;).width(0);
            $(&quot;#div_page&quot;).width(0);
            $(&quot;#div_footer&quot;).width(0);

            $(&quot;#div_filter&quot;).width($(&quot;#div_dummy_data&quot;).width());
            $(&quot;#div_data&quot;).width($(&quot;#div_dummy_data&quot;).width());
            $(&quot;#div_page&quot;).width($(&quot;#div_dummy_data&quot;).width());
            $(&quot;#div_footer&quot;).width($(&quot;#div_dummy_data&quot;).width());

            $(&quot;#div_container&quot;).css(&quot; , &quot;'&quot; , &quot;max-height&quot; , &quot;'&quot; , &quot;, $(window).height());

            let margin_height = 40;
            let filter_height = $(&quot; , &quot;'&quot; , &quot;#div_body&quot; , &quot;'&quot; , &quot;).height() - $(&quot; , &quot;'&quot; , &quot;#div_data&quot; , &quot;'&quot; , &quot;).height() + margin_height;
            $(&quot; , &quot;'&quot; , &quot;#div_data&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;max-height&quot; , &quot;'&quot; , &quot;, $(window).height() - $(&quot; , &quot;'&quot; , &quot;#div_header&quot; , &quot;'&quot; , &quot;).height() - $(&quot; , &quot;'&quot; , &quot;#div_page&quot; , &quot;'&quot; , &quot;).height() - $(&quot; , &quot;'&quot; , &quot;#div_footer&quot; , &quot;'&quot; , &quot;).height() - filter_height);
	}
	
	function confirmation(title, msg, fnYes, fnNo) {
            window.parent.Ext.Msg.show({
                title: title,
                msg: msg,
                buttons: window.parent.Ext.Msg.YESNO,
                icon: window.parent.Ext.Msg.QUESTION,
                fn: function(btn) {
                    if (btn === &quot; , &quot;'&quot; , &quot;yes&quot; , &quot;'&quot; , &quot;) {
                        fnYes();
                    } else {
                        fnNo();
                    }
                }
            });
	}

	function showFormReload(mypage, myname, w, h, scroll) {
            var openwin = window.parent.Ext.create(&quot; , &quot;'&quot; , &quot;Ext.window.Window&quot; , &quot;'&quot; , &quot;, {
                title: myname,
                collapsible: true,
                animCollapse: true,
                maximizable: true,
                closable: true,
                width: w,
                height: h,
                minWidth: w,
                minHeight: h,
                layout: &quot; , &quot;'&quot; , &quot;fit&quot; , &quot;'&quot; , &quot;,
                modal: true,
                html: &quot; , &quot;'&quot; , &quot;&lt;iframe src=&quot;&quot; , &quot;'&quot; , &quot; + mypage + &quot; , &quot;'&quot; , &quot;&quot;  frameborder=&quot;0&quot; style=&quot;border:0; height:100%; width:100%; overflow-y:hidden; overflow-x:hidden; overflow:hidden;&quot; scrolling=&quot;no&quot;>&lt;/iframe>&quot; , &quot;'&quot; , &quot;,
                listeners: {
                    close: function () {
                        resubmit();
                    },
                        destroy: function (wnd, eOpts) {
                    }
                }
            });
            openwin.show();
            return openwin;
	}

	function showFormRefilter(mypage, myname, w, h, scroll) {
            var openwin = window.parent.Ext.create(&quot; , &quot;'&quot; , &quot;Ext.window.Window&quot; , &quot;'&quot; , &quot;, {
                title: myname,
                collapsible: true,
                animCollapse: true,
                maximizable: true,
                closable: true,
                width: w,
                height: h,
                minWidth: w,
                minHeight: h,
                layout: &quot; , &quot;'&quot; , &quot;fit&quot; , &quot;'&quot; , &quot;,
                modal: true,
                html: &quot; , &quot;'&quot; , &quot;&lt;iframe src=&quot;&quot; , &quot;'&quot; , &quot; + mypage + &quot; , &quot;'&quot; , &quot;&quot;  frameborder=&quot;0&quot; style=&quot;border:0; height:100%; width:100%; overflow-y:hidden; overflow-x:hidden; overflow:hidden;&quot; scrolling=&quot;no&quot;>&lt;/iframe>&quot; , &quot;'&quot; , &quot;,
                listeners: {
                    close: function () {
                        filter();
                    },
                        destroy: function (wnd, eOpts) {
                    }
                }
            });
            openwin.show();
            return openwin;
	}

	function showForm(mypage, myname, w, h, scroll) {
            var openwin = window.parent.Ext.create(&quot; , &quot;'&quot; , &quot;Ext.window.Window&quot; , &quot;'&quot; , &quot;, {
                title: myname,
                collapsible: true,
                animCollapse: true,
                maximizable: true,
                closable: true,
                width: w,
                height: h,
                minWidth: w,
                minHeight: h,
                layout: &quot; , &quot;'&quot; , &quot;fit&quot; , &quot;'&quot; , &quot;,
                modal: true,
                html: &quot; , &quot;'&quot; , &quot;&lt;iframe src=&quot;&quot; , &quot;'&quot; , &quot; + mypage + &quot; , &quot;'&quot; , &quot;&quot;  frameborder=&quot;0&quot; style=&quot;border:0; height:100%; width:100%; overflow-y:hidden; overflow-x:hidden; overflow:hidden;&quot; scrolling=&quot;no&quot;>&lt;/iframe>&quot; , &quot;'&quot; , &quot;,
                listeners: {
                    close: function () {
                    },
                        destroy: function (wnd, eOpts) {
                    }
                }
            });
            openwin.show();
            return openwin;
	}

	function getBase64(file) {
            return new Promise((resolve, reject) => {
		const reader = new FileReader();
		reader.readAsDataURL(file);
		reader.onload = () => resolve(reader.result);
		reader.onerror = error => reject(error);
            });
	}
        
        function enableSelect2EditFailed(data_ke) {
            console.log(&quot;enableSelect2EditFailed: &quot;+data_ke);
            $(&quot;#tipe_input&quot;+data_ke+&quot;, #kategori&quot;+data_ke+&quot;, #seq&quot;+data_ke+&quot;, #status&quot;+data_ke).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;);
            $(&quot;#tipe_input&quot;+data_ke+&quot;, #kategori&quot;+data_ke+&quot;, #seq&quot;+data_ke+&quot;, #status&quot;+data_ke).select2({allowClear: true});
            $(&quot;#tipe_input&quot;+data_ke+&quot;, #kategori&quot;+data_ke+&quot;, #seq&quot;+data_ke+&quot;, #status&quot;+data_ke).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
            //$(&quot;#tipe_input&quot;+data_ke+&quot; select:first-of-type, #kategori&quot;+data_ke+&quot; select:first-of-type, #seq&quot;+data_ke+&quot; select:first-of-type, #status&quot;+data_ke+&quot; select:first-of-type&quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
            //$(&quot;#tipe_input&quot;+data_ke+&quot;:first-of-type, #kategori&quot;+data_ke+&quot;:first-of-type, #seq&quot;+data_ke+&quot;:first-of-type, #status&quot;+data_ke+&quot;:first-of-type&quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
            $(&quot;#tipe_input&quot;+data_ke+&quot;, #kategori&quot;+data_ke+&quot;, #seq&quot;+data_ke+&quot;, #status&quot;+data_ke).on(&quot; , &quot;'&quot; , &quot;select2:select&quot; , &quot;'&quot; , &quot;, function(e) {
                $(this).nextAll(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;).first().prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
            });

            /*$(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;).select2();
            $(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;).prop(&quot;disabled&quot;, true);
            $(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;).eq(0).prop(&quot;disabled&quot;, false);
            $(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;select2:select&quot; , &quot;'&quot; , &quot;, function(e) {
              var index = $(this).index(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;);
              $(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;).eq(index + 1).prop(&quot;disabled&quot;, false);
            });*/

            $(&quot;#tipe_input&quot;+data_ke+&quot;, #kategori&quot;+data_ke+&quot;, #seq&quot;+data_ke+&quot;, #status&quot;+data_ke).select2(&quot;enable&quot;, true);
        }
	
	async function save(parameter=&quot;&quot;){
            if (parameter===&quot;new&quot;) {
		confirmation(&quot; , &quot;'&quot; , &quot;Simpan Data&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Yakin untuk simpan data?&quot; , &quot;'&quot; , &quot;, async function(){
                    empty_field = 0;
                    length_field_100 = 0;
                    $(&quot; , &quot;'&quot; , &quot;input[type=text]&quot; , &quot;'&quot; , &quot;).each(function() {
                        if($(this).val() == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                           empty_field += 1;
                           $(this).css(&quot; , &quot;'&quot; , &quot;background-color&quot; , &quot;'&quot; , &quot; , &quot; , &quot;'&quot; , &quot;#f8c2b6&quot; , &quot;'&quot; , &quot;); //FF0000
                        } else {
                            if ($(this).val().length > 100) {
                                length_field_100++;
                                $(this).css(&quot; , &quot;'&quot; , &quot;background-color&quot; , &quot;'&quot; , &quot; , &quot; , &quot;'&quot; , &quot;#f8c2b6&quot; , &quot;'&quot; , &quot;); //FF0000
                            }
                        }
                    });
                    if (empty_field > 0) return alert(&quot; , &quot;'&quot; , &quot;Tidak boleh kosong&quot; , &quot;'&quot; , &quot;);
                    if (length_field_100 > 0) return alert(&quot; , &quot;'&quot; , &quot;Field size tidak boleh melebihi 100 karakter. Length saat ini: &quot; , &quot;'&quot; , &quot;+length_field_100);
                    
                    const form = $(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;)[0]; 
                    let formData = new FormData(form);
                    formData.append(&quot; , &quot;'&quot; , &quot;tipe&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;save_new&quot; , &quot;'&quot; , &quot;);

                    preloadMask(true);
                    $.ajax({
                        type: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;,
                        url: &quot;http://172.28.108.46:5454/smile/mod_ec/ajax/as1031_action.php?&quot;+Math.random(),
                        data: formData ? formData : form.serialize(),
                        contentType : false,
                        processData: false,
                        success: function(datas){
                            //alert(datas);
                            console.log(&quot;datas&quot;,datas);
                            jdata = JSON.parse(datas);
                            console.log(jdata);
                            console.log(&quot;jdata.length: &quot;+Object.keys(jdata).length);
                            if (Object.keys(jdata).length==1) {
                                if (jdata[0].ret == 1){
                                    alert(jdata[0].msg);
                                    //location.replace(&quot; , &quot;'&quot; , &quot;http://&lt; ?=$HTTP_HOST;?>/mod_ec/form/as1031.php?former_kategori_subkategori=&lt; ?=$former_kategori_subkategori?>&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&lt; ?=$former_search_txt?>&quot;+&quot;&amp;former_search_txt2=&lt; ?=$former_search_txt2?>&quot;+&quot;&amp;former_search_txt3=&lt; ?=$former_search_txt3?>&quot;);
                                    //location.replace(&quot; , &quot;'&quot; , &quot;http://172.28.108.46:5454/smile/mod_ec/form/as1031.php?former_kategori_subkategori=Y&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&quot;+&quot;&amp;former_search_txt2=&quot;+&quot;&amp;sess=div-container-table&quot;+&quot;&amp;selected_data=&quot;+&quot;&amp;status_crud=sukses&quot;);
                                    location.replace(&quot; , &quot;'&quot; , &quot;http://172.28.108.46:5454/smile/mod_ec/form/as1031.php?former_kategori_subkategori=Y&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&quot;+&quot;&amp;former_search_txt2=&quot;);
                                } else {
                                    alert(jdata[0].msg);
                                }
                            } else {
                                $(&quot;#saved_message&quot;).html(&quot;&quot;);
                                $(&quot;#saved_message&quot;).show();
                                var return_message = &quot;&quot;;
                                var failed = &quot;&quot;;
                                $.each(jdata, function(idx, val) {
                                    return_message = return_message + &quot;Data ke-&quot; + val.ke + &quot;: &quot; + val.msg + &quot;\n\r&lt;br/>&quot;;
                                    if (val.ret==&quot;1&quot;) {
                                        $(&quot;#saved_message&quot;).append(&quot;&lt;span class=&quot; , &quot;'&quot; , &quot;saved_message_success&quot; , &quot;'&quot; , &quot;>idx ke: &quot;+idx+&quot;, val ke: &quot;+val.ke+&quot;, val ret: &quot;+val.ret+&quot;, val msg: &quot;+val.msg+&quot;, val no urut: &quot;+val.noUrut+&quot;, val input: &quot;+val.input+&quot;, val status: &quot;+val.status+&quot;&lt;span>&lt;br/>&lt;br/>                        \n\n\n\n&quot;);
                                        $(&quot;#saved_message&quot;).append(&quot;\n\r&quot;);
                                        $(&quot;#saved_message&quot;).append(&quot;\n\r&quot;);
                                        
                                        div_color = &quot;&lt;div style=&quot; , &quot;'&quot; , &quot;z-index:2;background: linear-gradient(159deg, rgba(173,223,173,1) 0%, rgba(50,205,50,1) 100%);opacity:0.3;position:absolute;border-radius:10px;left:0px;top:0px;right:0px;bottom:0px;&quot; , &quot;'&quot; , &quot;>&lt;/div>&quot;;
                                        if (idx==0) {
                                            $(&quot;.konten_form0&quot;).append(div_color);
                                            $(&quot;.konten_form0 input[type=text]&quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
                                            $(&quot;.konten_form0 input[type=text]&quot;).attr(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;);
                                            $(&quot;.konten_form0 select&quot;).select2(&quot;enable&quot;, false);
                                        } else {
                                            $(&quot;.konten_form&quot;+(idx+1)).append(div_color);
                                            $(&quot;.konten_form&quot;+(idx+1)+&quot; input[type=text]&quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
                                            $(&quot;.konten_form&quot;+(idx+1)+&quot; input[type=text]&quot;).attr(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;);
                                            $(&quot;.konten_form&quot;+(idx+1)+&quot; select&quot;).select2(&quot;enable&quot;, false);
                                        }
                                    } else {
                                        $(&quot;#saved_message&quot;).append(&quot;&lt;span class=&quot; , &quot;'&quot; , &quot;saved_message_failed&quot; , &quot;'&quot; , &quot;>idx ke: &quot;+idx+&quot;, val ke: &quot;+val.ke+&quot;, val ret: &quot;+val.ret+&quot;, val msg: &quot;+val.msg+&quot;, val no urut: &quot;+val.noUrut+&quot;, val input: &quot;+val.input+&quot;, val status: &quot;+val.status+&quot;&lt;span>&lt;br/>&lt;br/>                        \n\n\n\n&quot;);
                                        $(&quot;#saved_message&quot;).append(&quot;\n\r&quot;);
                                        $(&quot;#saved_message&quot;).append(&quot;\n\r&quot;);
                                        failed += failed + &quot;Data ke-&quot; + val.ke + &quot;: &quot; + val.msg + &quot;\n\r&lt;br/>&quot;;
                                        
                                        if (idx==0) {
                                            $(&quot;.konten_form0 input[type=text]&quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
                                            $(&quot;.konten_form0 input[type=text]&quot;).removeAttr(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;);
                                            //$(&quot;.konten_form0 select&quot;).select2(&quot;enable&quot;, true);
                                        } else {
                                            $(&quot;.konten_form&quot;+(idx+1)+&quot; input[type=text]&quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
                                            $(&quot;.konten_form&quot;+(idx+1)+&quot; input[type=text]&quot;).removeAttr(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;);
                                            //$(&quot;.konten_form&quot;+(idx+1)+&quot; select&quot;).select2(&quot;enable&quot;, true);
                                        }
                                    }
                                });
                                alert(return_message);
                                if (failed==&quot;&quot;) {
                                    //location.replace(&quot; , &quot;'&quot; , &quot;http://&lt; ?=$HTTP_HOST;?>/mod_ec/form/as1031.php?former_kategori_subkategori=&lt; ?=$former_kategori_subkategori?>&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&lt; ?=$former_search_txt?>&quot;+&quot;&amp;former_search_txt2=&lt; ?=$former_search_txt2?>&quot;+&quot;&amp;former_search_txt3=&lt; ?=$former_search_txt3?>&quot;);
                                    //location.replace(&quot; , &quot;'&quot; , &quot;http://&lt; ?=$HTTP_HOST;?>/mod_ec/form/as1031.php?former_kategori_subkategori=&lt; ?=$former_kategori_subkategori?>&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&lt; ?=$former_search_txt?>&quot;+&quot;&amp;former_search_txt2=&lt; ?=$former_search_txt2?>&quot;+&quot;&amp;sess=&lt; ?=$former_sess_table?>&quot;+&quot;&amp;selected_data=&lt; ?=$former_selected_data?>&quot;+&quot;&amp;status_crud=sukses&quot;);
                                    location.replace(&quot; , &quot;'&quot; , &quot;http://172.28.108.46:5454/smile/mod_ec/form/as1031.php?former_kategori_subkategori=Y&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&quot;+&quot;&amp;former_search_txt2=&quot;);
                                }
                            }
                            
//                            if (jdata.ret == 1){
//                                alert(jdata.msg);
//                                //location.replace(&quot; , &quot;'&quot; , &quot;http://&lt; ?= $HTTP_HOST; ?>/mod_ec/form/as1031.php?former_kategori_subkategori=&lt; ?=$former_kategori_subkategori?>&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&lt; ?=$former_search_txt?>&quot;+&quot;&amp;former_search_txt2=&lt; ?=$former_search_txt2?>&quot;+&quot;&amp;former_search_txt3=&lt; ?=$former_search_txt3?>&quot;);
//                            } else {
//                                alert(jdata.msg);
//                            }
                            preloadMask(false);
                        },
                        complete: function(){
                            preloadMask(false);
                        },
                        error: function(){
                            alert(&quot;Terjadi kesalahan, coba beberapa saat lagi!&quot;);
                            preloadMask(false);
                        }
                    });
		}, function(){});
            } else if (parameter===&quot;edit&quot;) {
		confirmation(&quot; , &quot;'&quot; , &quot;Edit Data&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Yakin untuk edit data?&quot; , &quot;'&quot; , &quot;, async function(){
                    empty_field = 0;
                    length_field_100 = 0;
                    $(&quot; , &quot;'&quot; , &quot;input[type=text]&quot; , &quot;'&quot; , &quot;).each(function() {
                        if($(this).val() == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                           empty_field += 1;
                           $(this).css(&quot; , &quot;'&quot; , &quot;background-color&quot; , &quot;'&quot; , &quot; , &quot; , &quot;'&quot; , &quot;#f8c2b6&quot; , &quot;'&quot; , &quot;); //FF0000
                        } else {
                            if ($(this).val().length > 100) {
                                length_field_100++;
                                $(this).css(&quot; , &quot;'&quot; , &quot;background-color&quot; , &quot;'&quot; , &quot; , &quot; , &quot;'&quot; , &quot;#f8c2b6&quot; , &quot;'&quot; , &quot;); //FF0000
                            }
                        }
                    });
                    if (empty_field > 0) return alert(&quot; , &quot;'&quot; , &quot;Tidak boleh kosong&quot; , &quot;'&quot; , &quot;);
                    if (length_field_100 > 0) return alert(&quot; , &quot;'&quot; , &quot;Field size tidak boleh melebihi 100 karakter. Length saat ini: &quot; , &quot;'&quot; , &quot;+length_field_100);
                    
                    const form = $(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;)[0]; 
                    let formData = new FormData(form);
                    formData.append(&quot; , &quot;'&quot; , &quot;tipe&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;save_edit&quot; , &quot;'&quot; , &quot;);

                    preloadMask(true);
                    $.ajax({
                        type: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;,
                        url: &quot;http://172.28.108.46:5454/smile/mod_ec/ajax/as1031_action.php?&quot;+Math.random(),
                        data: formData ? formData : form.serialize(),
                        contentType : false,
                        processData: false,
                        success: function(datas){
                            //alert(datas);
                            console.log(&quot;datas&quot;,datas);
                            sessionStorage.setItem(&quot;kategori_subkategori_save_edit&quot;, datas);
                            console.log(&quot;datas kategori_subkategori_save_edit defined&quot;,datas);
                            jdata = JSON.parse(datas);
                            console.log(jdata);
                            console.log(&quot;jdata.length: &quot;+Object.keys(jdata).length);
                            if (Object.keys(jdata).length==1) {
                                if (jdata[0].ret == 1){
                                    alert(jdata[0].msg);
                                    //location.replace(&quot; , &quot;'&quot; , &quot;http://&lt; ?=$HTTP_HOST;?>/mod_ec/form/as1031.php?former_kategori_subkategori=&lt; ?=$former_kategori_subkategori?>&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&lt; ?=$former_search_txt?>&quot;+&quot;&amp;former_search_txt2=&lt; ?=$former_search_txt2?>&quot;+&quot;&amp;former_search_txt3=&lt; ?=$former_search_txt3?>&quot;);
                                    location.replace(&quot; , &quot;'&quot; , &quot;http://172.28.108.46:5454/smile/mod_ec/form/as1031.php?former_kategori_subkategori=Y&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&quot;+&quot;&amp;former_search_txt2=&quot;+&quot;&amp;sess=div-container-table&quot;+&quot;&amp;selected_data=&quot;+&quot;&amp;status_crud=sukses&quot;);
                                } else {
                                    alert(jdata[0].msg);
                                }
                            } else {
                                $(&quot;#saved_message&quot;).html(&quot;&quot;);
                                $(&quot;#saved_message&quot;).show();
                                var return_message = &quot;&quot;;
                                var failed = &quot;&quot;;
                                $.each(jdata, function(idx, val) {
                                    let data_ke = idx + 1;
                                    return_message = return_message + &quot;Data ke-&quot; + val.ke + &quot;: &quot; + val.msg + &quot;\n\r&lt;br/>&quot;;
                                    if (val.ret==&quot;1&quot;) {
                                        console.log(&quot;idx: &quot;+idx);
                                        $(&quot;#saved_message&quot;).append(&quot;&lt;span class=&quot; , &quot;'&quot; , &quot;saved_message_success&quot; , &quot;'&quot; , &quot;>idx ke: &quot;+data_ke+&quot;, val ke: &quot;+val.ke+&quot;, val ret: &quot;+val.ret+&quot;, val msg: &quot;+val.msg+&quot;, val no urut: &quot;+val.noUrut+&quot;, val input: &quot;+val.input+&quot;, val status: &quot;+val.status+&quot;&lt;/span>&lt;br/>&lt;br/>                        \n\n\n\n&quot;);
                                        $(&quot;#saved_message&quot;).append(&quot;\n\r&quot;);
                                        $(&quot;#saved_message&quot;).append(&quot;\n\r&quot;);
                                    
                                        let div_color = &quot;&lt;div style=&quot; , &quot;'&quot; , &quot;z-index:2;background: linear-gradient(159deg, rgba(173,223,173,1) 0%, rgba(50,205,50,1) 100%);opacity:0.3;position:absolute;border-radius:10px;left:0px;top:0px;right:0px;bottom:0px;&quot; , &quot;'&quot; , &quot;>&lt;/div>&quot;;
                                        $(&quot;.konten_form&quot;+val.ke).append(div_color);
                                        $(&quot;.konten_form&quot;+val.ke+&quot; input[type=text], .konten_form&quot;+val.ke+&quot; input[type=hidden]&quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
                                        $(&quot;.konten_form&quot;+val.ke+&quot; input[type=text], .konten_form&quot;+val.ke+&quot; input[type=hidden]&quot;).attr(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;);
                                        $(&quot;.konten_form&quot;+val.ke+&quot; select&quot;).select2(&quot;enable&quot;, false);
                                        console.log(&quot;data_ke: &quot;+data_ke);
                                    } else {
                                        $(&quot;#saved_message&quot;).append(&quot;&lt;span class=&quot; , &quot;'&quot; , &quot;saved_message_failed&quot; , &quot;'&quot; , &quot;>idx ke: &quot;+data_ke+&quot;, val ke: &quot;+val.ke+&quot;, val ret: &quot;+val.ret+&quot;, val msg: &quot;+val.msg+&quot;, val no urut: &quot;+val.noUrut+&quot;, val input: &quot;+val.input+&quot;, val status: &quot;+val.status+&quot;&lt;/span>&lt;br/>&lt;br/>                        \n\n\n\n&quot;);
                                        $(&quot;#saved_message&quot;).append(&quot;\n\r&quot;);
                                        $(&quot;#saved_message&quot;).append(&quot;\n\r&quot;);
                                        //$(&quot;.konten_form&quot;+val.ke).append(&quot;&lt;input type=&quot; , &quot;'&quot; , &quot;button&quot; , &quot;'&quot; , &quot; onclick=&quot; , &quot;'&quot; , &quot;enableSelect2EditFailed(&quot;+val.ke+&quot;);&quot; , &quot;'&quot; , &quot; value=&quot; , &quot;'&quot; , &quot;Enable Select2&quot; , &quot;'&quot; , &quot; />&quot;);
                                        failed += failed + &quot;Data ke-&quot; + val.ke + &quot;: &quot; + val.msg + &quot;\n\r&lt;br/>&quot;;
                                        
                                        $(&quot;.konten_form&quot;+val.ke+&quot; input[type=hidden]&quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
                                        $(&quot;.konten_form&quot;+val.ke+&quot; input[type=hidden]&quot;).removeAttr(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;, false);
                                        $(&quot;.konten_form&quot;+val.ke+&quot; #subkategori&quot;+val.ke).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
                                        $(&quot;.konten_form&quot;+val.ke+&quot; #subkategori&quot;+val.ke).removeAttr(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;, false);
                                        //$(&quot;select&quot;).select2(&quot;enable&quot;, true);
                                        $(&quot;#kode_subkategori&quot;+val.ke).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;); //hidden
                                        $(&quot;#kode_subkategori&quot;+val.ke).prop(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;, false).removeAttr(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;); //hidden
                                    }
                                });
                                alert(return_message);
                                if (failed==&quot;&quot;) {
                                    //location.replace(&quot; , &quot;'&quot; , &quot;http://&lt; ?=$HTTP_HOST;?>/mod_ec/form/as1031.php?former_kategori_subkategori=&lt; ?=$former_kategori_subkategori?>&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&lt; ?=$former_search_txt?>&quot;+&quot;&amp;former_search_txt2=&lt; ?=$former_search_txt2?>&quot;+&quot;&amp;former_search_txt3=&lt; ?=$former_search_txt3?>&quot;);
                                    location.replace(&quot; , &quot;'&quot; , &quot;http://172.28.108.46:5454/smile/mod_ec/form/as1031.php?former_kategori_subkategori=Y&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&quot;+&quot;&amp;former_search_txt2=&quot;+&quot;&amp;sess=div-container-table&quot;+&quot;&amp;selected_data=&quot;+&quot;&amp;status_crud=sukses&quot;);
                                }
                            }
                            preloadMask(false);
                        },
                        complete: function(){
                            preloadMask(false);
                        },
                        error: function(){
                            alert(&quot;Terjadi kesalahan, coba beberapa saat lagi!&quot;);
                            preloadMask(false);
                        }
                    });
		}, function(){});
            }
	} 

	function kembali(){
            //window.location.replace(&quot; , &quot;'&quot; , &quot;http://&lt; ?=$HTTP_HOST; ?>/mod_ec/form/as1031.php?former_kategori_subkategori=&lt; ?=$former_kategori_subkategori?>&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&lt; ?=$former_search_txt?>&quot;+&quot;&amp;former_search_txt2=&lt; ?=$former_search_txt2?>&quot;+&quot;&amp;former_search_txt3=&lt; ?=$former_search_txt3?>&quot;);
            window.location.replace(&quot; , &quot;'&quot; , &quot;http://172.28.108.46:5454/smile/mod_ec/form/as1031.php?former_kategori_subkategori=Y&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&quot;+&quot;&amp;former_search_txt2=&quot;+&quot;&amp;sess=div-container-table&quot;+&quot;&amp;selected_data=&quot;);
	} 
		
	function emptyrows() {
            let html_data = &quot;&quot;;
            html_data += &quot; , &quot;'&quot; , &quot;&lt;tr class=&quot;nohover-color&quot;>&quot; , &quot;'&quot; , &quot;;
            html_data += &quot; , &quot;'&quot; , &quot;&lt;td colspan=&quot;10&quot; style=&quot;text-align: center;&quot;>-- Data tidak ditemukan --&lt;/td>&quot; , &quot;'&quot; , &quot;;
            html_data += &quot; , &quot;'&quot; , &quot;&lt;/tr>&quot; , &quot;'&quot; , &quot;;
            $(&quot;#data_list&quot;).html(html_data);
	}

	function download() {
            const linkDownload  = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            window.open(linkDownload, &quot; , &quot;'&quot; , &quot;_blank&quot; , &quot;'&quot; , &quot;).focus();
            //location.replace(&quot; , &quot;'&quot; , &quot;http://&lt; ?= $HTTP_HOST; ?>/mod_ec/form/as1031.php?former_kategori_subkategori=&lt; ?=$former_kategori_subkategori?>&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&lt; ?=$former_search_txt?>&quot;+&quot;&amp;former_search_txt2=&quot;+&quot;&amp;former_search_txt3=&lt; ?=$former_search_txt3?>&quot;);
            location.replace(&quot; , &quot;'&quot; , &quot;http://172.28.108.46:5454/smile/mod_ec/form/as1031.php?former_kategori_subkategori=Y&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&quot;+&quot;&amp;former_search_txt2=&quot;);
	}

	function regenerate(id_batch) {
            preloadMask(true);
            $.ajax({
                type: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;,
                url: &quot;http://172.28.108.46:5454/smile/mod_ec/ajax/as1031_action.php?&quot;+Math.random(),
                data: {
                    tipe: &quot; , &quot;'&quot; , &quot;regenerate&quot; , &quot;'&quot; , &quot;,
                    idBatch: id_batch
                },
                success: function(datas){
                    console.log(&quot;datas&quot;,datas);
                    jdata = JSON.parse(datas);
                    console.log(jdata);
                    if (jdata.ret == 1){
                        alert(jdata.msg);
                        //location.replace(&quot; , &quot;'&quot; , &quot;http://&lt; ?=$HTTP_HOST; ?>/mod_ec/form/as1031.php?former_kategori_subkategori=&lt; ?=$former_kategori_subkategori?>&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&lt; ?=$former_search_txt?>&quot;+&quot;&amp;former_search_txt2=&lt; ?=$former_search_txt2?>&quot;+&quot;&amp;former_search_txt3=&lt; ?=$former_search_txt3?>&quot;);
                        location.replace(&quot; , &quot;'&quot; , &quot;http://172.28.108.46:5454/smile/mod_ec/form/as1031.php?former_kategori_subkategori=Y&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&quot;+&quot;&amp;former_search_txt2=&quot;+&quot;&amp;sess=div-container-table&quot;+&quot;&amp;selected_data=&quot;+&quot;&amp;status_crud=sukses&quot;);
                    } else {
                        alert(jdata.msg);
                    }
                    preloadMask(false);
                },
                complete: function(){
                    preloadMask(false);
                },
                error: function(){
                    alert(&quot;Terjadi kesalahan, coba beberapa saat lagi!&quot;);
                    preloadMask(false);
                }
            });
	}
        
        function addInputs(kode_konten, mode, obj) { //T = KATEGORI, Y = SUB KATEGORI
            /*$(&quot; , &quot;'&quot; , &quot;#konten_form&quot; , &quot;'&quot; , &quot;).clone().insertAfter(&quot; , &quot;'&quot; , &quot;div#konten2:last&quot; , &quot;'&quot; , &quot;);*/
            //$(&quot;div[id^=&quot; , &quot;'&quot; , &quot;konten_form&quot; , &quot;'&quot; , &quot;]:last&quot;).after($(&quot; , &quot;'&quot; , &quot;div#konten2&quot; , &quot;'&quot; , &quot;).clone());
            //alert(&quot; , &quot;'&quot; , &quot;test: &quot; , &quot;'&quot; , &quot;+kode_konten);
            
            //alert($(&quot;div#konten_form&quot;).length);
//            if (mode==&quot;change&quot;) {
//                //$(obj+&quot;.tipe_input,&quot;+obj+&quot;.kategori,&quot;+obj+&quot;.subkategori,&quot;+obj+&quot;.seq,&quot;+obj+&quot;.status&quot;).select2(&quot;destroy&quot;);
//                $(obj).remove();
//            }
            
            var no_konten_form = $(&quot;div#konten_form&quot;).length + 1;
            if (no_konten_form==11) {
                alert(&quot;Anda hanya bisa menambahkan 10 data.&quot;);
                return;
            }
            
            var no_urut = &quot;&quot;;
            for (let i = 1; i &lt;= 100; i++) {
                var selected = &quot;&quot;;
                if (no_konten_form==i) {
                    selected = &quot;selected=&quot; , &quot;'&quot; , &quot;selected&quot; , &quot;'&quot; , &quot;&quot;;
                }
                no_urut += &quot;&lt;option value=&quot; , &quot;'&quot; , &quot;&quot;+i+&quot;&quot; , &quot;'&quot; , &quot; &quot;+selected+&quot;>&quot;+i+&quot;&lt;/option>&quot;;
            }
            
            var input = &quot; , &quot;'&quot; , &quot;&lt;div id=&quot;konten_form&quot; class=&quot;konten_form&quot; , &quot;'&quot; , &quot;+no_konten_form+&quot; , &quot;'&quot; , &quot;&quot; style=&quot;float:left;position:relative;border-radius:10px;width:440px;height:128px;border:1px solid black;margin:10px;padding:40px 10px 10px 10px;&quot;>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div style=&quot;z-index:2;color:white;background: -webkit-linear-gradient(left,#6ba5ff,#416fd6);position:absolute;border-top-left-radius:10px;border-top-right-radius:10px;padding:6px 2px 2px 2px;font-weight:bold;font-size:12px;height:20px;left:0px;top:0px;right:0px;text-align:center;&quot;>Data Ke-&quot; , &quot;'&quot; , &quot;+no_konten_form+&quot; , &quot;'&quot; , &quot;&lt;img style=&quot;float:right;width:15px;height:auto;margin-right:4px;cursor:pointer;&quot; src=&quot;../../images/removex.png&quot; onclick=&quot;$(\&quot; , &quot;'&quot; , &quot;.konten_form&quot; , &quot;'&quot; , &quot;+no_konten_form+&quot; , &quot;'&quot; , &quot;\&quot; , &quot;'&quot; , &quot;).remove();&quot; />&lt;/div>&quot; , &quot;'&quot; , &quot;;
            if (kode_konten==&quot;T&quot; || kode_konten==&quot;KATEGORI&quot;) { //kategori
                    input += &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;form-row_kiri&quot;>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;label style= &quot;text-align:right;width: 130px;&quot;>Tipe &lt;span style=&quot;color:#ff0000;&quot;>*&lt;/span> :&lt;/label>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;select name=&quot;tipe_input[]&quot; class=&quot;tipe_input required&quot; style=&quot;width: 250px;&quot; accept=&quot;text/csv&quot; onchange=&quot;addInputs($(this).val(), \&quot; , &quot;'&quot; , &quot;change\&quot; , &quot;'&quot; , &quot;, \&quot; , &quot;'&quot; , &quot;.konten_form&quot; , &quot;'&quot; , &quot;+no_konten_form+&quot; , &quot;'&quot; , &quot;\&quot; , &quot;'&quot; , &quot;)&quot;>&quot; , &quot;'&quot; , &quot;+
                                            &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;KATEGORI&quot; selected=&quot;selected&quot;>KATEGORI&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                            //&quot; , &quot;'&quot; , &quot;&lt;option value=&quot;SUB KATEGORI&quot;>SUB KATEGORI&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;/select>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;clear&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;form-row_kiri&quot;>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;label style= &quot;text-align:right;width: 130px;&quot;>Nama Kategori &lt;span style=&quot;color:#ff0000;&quot;>*&lt;/span> :&lt;/label>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;input type=&quot;text&quot; name=&quot;kategori[]&quot; class=&quot;kategori required&quot; style=&quot;width: 245px;&quot; accept=&quot;text/csv&quot; required oninvalid=&quot;this.setCustomValidity(\&quot; , &quot;'&quot; , &quot;Input nama kategori.\&quot; , &quot;'&quot; , &quot;)&quot; oninput=&quot;this.setCustomValidity(\&quot; , &quot;'&quot; , &quot;\&quot; , &quot;'&quot; , &quot;); $(this).css(\&quot; , &quot;'&quot; , &quot;background-color\&quot; , &quot;'&quot; , &quot; , \&quot; , &quot;'&quot; , &quot;#FFFFFF\&quot; , &quot;'&quot; , &quot;);&quot; />&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;clear&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;form-row_kiri&quot;>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;label style= &quot;text-align:right;width: 130px;&quot;>No Urut &lt;span style=&quot;color:#ff0000;&quot;>*&lt;/span> :&lt;/label>&quot; , &quot;'&quot; , &quot;+
                                    //&quot; , &quot;'&quot; , &quot;&lt;select name=&quot;kategori[]&quot; id=&quot;kategori0&quot; class=&quot;kategori required&quot; style=&quot;width: 250px;display:none;&quot; accept=&quot;text/csv&quot;>&quot; , &quot;'&quot; , &quot;+
                                    //&quot; , &quot;'&quot; , &quot;&lt;/select>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;select name=&quot;seq[]&quot; class=&quot;seq required&quot; style=&quot;width: 250px;&quot; accept=&quot;text/csv&quot;>&quot; , &quot;'&quot; , &quot;+
                                        no_urut+
                                    &quot; , &quot;'&quot; , &quot;&lt;/select>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;clear&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;form-row_kiri&quot;>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;label style= &quot;text-align:right;width: 130px;&quot;>Status &lt;span style=&quot;color:#ff0000;&quot;>*&lt;/span> :&lt;/label>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;select name=&quot;status[]&quot; class=&quot;status required&quot; style=&quot;width: 250px;&quot; accept=&quot;text/csv&quot;>&quot; , &quot;'&quot; , &quot;+
                                            &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;N&quot; selected=&quot;selected&quot;>TAMBAH DATA KATEGORI&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                            &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;Y&quot; disabled=&quot;disabled&quot;>AKTIF&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                            &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;T&quot; disabled=&quot;disabled&quot;>NON AKTIF&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                            &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;SF0&quot; disabled=&quot;disabled&quot;>PENYESUAIAN NAMA KATEGORI&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                            &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;SF1&quot; disabled=&quot;disabled&quot;>PENYESUAIAN NOMOR URUT&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;/select>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;+
                        &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;;
                $(&quot;#konten2&quot;).append(input);
                $(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;).select2();
                //$(&quot; , &quot;'&quot; , &quot;select.tipe_input, select.kategori, select.seq, select.status&quot; , &quot;'&quot; , &quot;).select2();
                $(&quot; , &quot;'&quot; , &quot;select.tipe_input, select.seq, select.status&quot; , &quot;'&quot; , &quot;).select2();
            } else if (kode_konten==&quot;Y&quot; || kode_konten==&quot;SUB KATEGORI&quot;) { //sub kategori
                /*var no_urut2 = &quot;&quot;;
                for (let i = 1; i &lt;= 100; i++) {
                    no_urut2 += &quot;&lt;option value=&quot; , &quot;'&quot; , &quot;&quot;+i+&quot;&quot; , &quot;'&quot; , &quot;>KATEGORI &quot;+i+&quot;&lt;/option>&quot;;
                }
                //no_urut2 = $(&quot; , &quot;'&quot; , &quot;#konten_form.kategori:first&quot; , &quot;'&quot; , &quot;).html();*/
                //alert(no_urut2);
                
                    input += &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;form-row_kiri&quot;>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;label style= &quot;text-align:right;width: 130px;&quot;>Tipe &lt;span style=&quot;color:#ff0000;&quot;>*&lt;/span> :&lt;/label>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;select name=&quot;tipe_input[]&quot; class=&quot;tipe_input required&quot; style=&quot;width: 250px;&quot; accept=&quot;text/csv&quot; onchange=&quot;addInputs($(this).val(), \&quot; , &quot;'&quot; , &quot;change\&quot; , &quot;'&quot; , &quot;, \&quot; , &quot;'&quot; , &quot;.konten_form&quot; , &quot;'&quot; , &quot;+no_konten_form+&quot; , &quot;'&quot; , &quot;\&quot; , &quot;'&quot; , &quot;)&quot;>&quot; , &quot;'&quot; , &quot;+
                                            //&quot; , &quot;'&quot; , &quot;&lt;option value=&quot;KATEGORI&quot;>KATEGORI&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                            &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;SUB KATEGORI&quot; selected=&quot;selected&quot;>SUB KATEGORI&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;/select>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;clear&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;form-row_kiri&quot;>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;label style= &quot;text-align:right;width: 130px;&quot;>Nama Kategori &lt;span style=&quot;color:#ff0000;&quot;>*&lt;/span> :&lt;/label>&quot; , &quot;'&quot; , &quot;+
                                    //&quot; , &quot;'&quot; , &quot;&lt;select name=&quot;kategori[]&quot; class=&quot;kategori&quot; style=&quot;width: 250px;&quot; accept=&quot;text/csv&quot;>&lt;/select>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;div id=&quot;tool_placeholder_&quot; , &quot;'&quot; , &quot;+no_konten_form+&quot; , &quot;'&quot; , &quot;&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;clear&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;form-row_kiri&quot;>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;label style= &quot;text-align:right;width: 130px;&quot;>Nama Sub Kategori &lt;span style=&quot;color:#ff0000;&quot;>*&lt;/span> :&lt;/label>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;input type=&quot;text&quot; name=&quot;subkategori[]&quot; class=&quot;subkategori required&quot; style=&quot;width: 245px;&quot; accept=&quot;text/csv&quot; required oninvalid=&quot;this.setCustomValidity(\&quot; , &quot;'&quot; , &quot;Input nama sub kategori.\&quot; , &quot;'&quot; , &quot;)&quot; oninput=&quot;this.setCustomValidity(\&quot; , &quot;'&quot; , &quot;\&quot; , &quot;'&quot; , &quot;); $(this).css(\&quot; , &quot;'&quot; , &quot;background-color\&quot; , &quot;'&quot; , &quot; , \&quot; , &quot;'&quot; , &quot;#FFFFFF\&quot; , &quot;'&quot; , &quot;);&quot; />&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;clear&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;form-row_kiri&quot;>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;label style= &quot;text-align:right;width: 130px;&quot;>No Urut &lt;span style=&quot;color:#ff0000;&quot;>*&lt;/span> :&lt;/label>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;select name=&quot;seq[]&quot; class=&quot;seq required&quot; style=&quot;width: 250px;&quot; accept=&quot;text/csv&quot;>&quot; , &quot;'&quot; , &quot;+
                                        no_urut+
                                    &quot; , &quot;'&quot; , &quot;&lt;/select>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;clear&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;form-row_kiri&quot;>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;label style= &quot;text-align:right;width: 130px;&quot;>Status &lt;span style=&quot;color:#ff0000;&quot;>*&lt;/span> :&lt;/label>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;select name=&quot;status[]&quot; class=&quot;status required&quot; style=&quot;width: 250px;&quot; accept=&quot;text/csv&quot;>&quot; , &quot;'&quot; , &quot;+
                                            &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;N&quot; selected=&quot;selected&quot;>TAMBAH DATA SUB KATEGORI&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                            &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;Y&quot; disabled=&quot;disabled&quot;>AKTIF&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                            &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;T&quot; disabled=&quot;disabled&quot;>NON AKTIF&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                            &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;SF0&quot; disabled=&quot;disabled&quot;>PENYESUAIAN NAMA KATEGORI&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                            &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;SF0&quot; disabled=&quot;disabled&quot;>PENYESUAIAN NAMA SUB KATEGORI&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                            &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;SF1&quot; disabled=&quot;disabled&quot;>PENYESUAIAN NOMOR URUT&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;/select>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;+
                        &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;;
                $(&quot;#konten2&quot;).append(input);
                
                //$(&quot; , &quot;'&quot; , &quot;select.kategori&quot; , &quot;'&quot; , &quot;).select2(&quot;destroy&quot;);
                var noOfSelect2 = $(&quot; , &quot;'&quot; , &quot;.kategori&quot; , &quot;'&quot; , &quot;).length;
                //var clonedSelect2 = $(&quot; , &quot;'&quot; , &quot;.kategori&quot; , &quot;'&quot; , &quot;).first().clone(true);
                var clonedSelect2 = $(&quot; , &quot;'&quot; , &quot;#kategori0&quot; , &quot;'&quot; , &quot;).clone(true);
                clonedSelect2.insertBefore(&quot;#tool_placeholder_&quot;+no_konten_form);
                clonedSelect2.attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;kategori&quot; , &quot;'&quot; , &quot;+noOfSelect2);
                clonedSelect2.attr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;kategori[]&quot; , &quot;'&quot; , &quot;);
                clonedSelect2.attr(&quot; , &quot;'&quot; , &quot;class&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;kategori&quot; , &quot;'&quot; , &quot;);
                clonedSelect2.attr(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;width: 250px;&quot; , &quot;'&quot; , &quot;);
                clonedSelect2.attr(&quot; , &quot;'&quot; , &quot;accept&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;text/csv&quot; , &quot;'&quot; , &quot;);
                $(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;).select2();
                //$(&quot; , &quot;'&quot; , &quot;select.kategori&quot; , &quot;'&quot; , &quot;).select2();
                $(&quot; , &quot;'&quot; , &quot;select#kategori&quot; , &quot;'&quot; , &quot;+noOfSelect2).select2();
                $(&quot; , &quot;'&quot; , &quot;select.tipe_input, select.kategori, select.seq, select.status&quot; , &quot;'&quot; , &quot;).select2();
            }
            
            $(&quot; , &quot;'&quot; , &quot;.kategori&quot; , &quot;'&quot; , &quot;).keypress(function (e) {
                console.log(&quot;keypress button: e.key=&quot;+e.key+&quot; - e.code=&quot;+e.code+&quot; - e.which=&quot;+e.which);
                /*if (e.which === 44 || e.key === &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&lt;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;>&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;?&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&quot;&quot; , &quot;'&quot; , &quot; || e.key === &quot;&quot; , &quot;'&quot; , &quot;&quot; || e.key === &quot; , &quot;'&quot; , &quot;{&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;[&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;}&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;|&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;\\&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;`&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;~&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;!&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;@&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;$&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;%&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;^&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;*&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;(&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;)&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;_&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;+&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;=&quot; , &quot;'&quot; , &quot;) {*/
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                    //alert(&quot;comma detected1&quot;);
                /*} else {
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replaceAll(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                }*/
            }).keyup(function (e) {
                /*if (e.which === 44 || e.key === &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&lt;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;>&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;?&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&quot;&quot; , &quot;'&quot; , &quot; || e.key === &quot;&quot; , &quot;'&quot; , &quot;&quot; || e.key === &quot; , &quot;'&quot; , &quot;{&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;[&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;}&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;|&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;\\&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;`&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;~&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;!&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;@&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;$&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;%&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;^&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;*&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;(&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;)&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;_&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;+&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;=&quot; , &quot;'&quot; , &quot;) {*/
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                    //alert(&quot;comma detected1&quot;);
                /*} else {
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replaceAll(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                }*/
            });
            
            $(&quot; , &quot;'&quot; , &quot;.subkategori&quot; , &quot;'&quot; , &quot;).keypress(function (e) {
                console.log(&quot;keypress button: e.key=&quot;+e.key+&quot; - e.code=&quot;+e.code+&quot; - e.which=&quot;+e.which);
                /*if (e.which === 44 || e.key === &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&lt;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;>&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;?&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&quot;&quot; , &quot;'&quot; , &quot; || e.key === &quot;&quot; , &quot;'&quot; , &quot;&quot; || e.key === &quot; , &quot;'&quot; , &quot;{&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;[&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;}&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;|&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;\\&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;`&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;~&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;!&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;@&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;$&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;%&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;^&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;*&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;(&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;)&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;_&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;+&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;=&quot; , &quot;'&quot; , &quot;) {*/
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                    //alert(&quot;comma detected1&quot;);
                /*} else {
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                }*/
            }).keyup(function (e) {
                /*if (e.which === 44 || e.key === &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&lt;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;>&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;?&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&quot;&quot; , &quot;'&quot; , &quot; || e.key === &quot;&quot; , &quot;'&quot; , &quot;&quot; || e.key === &quot; , &quot;'&quot; , &quot;{&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;[&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;}&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;|&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;\\&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;`&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;~&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;!&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;@&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;$&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;%&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;^&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;*&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;(&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;)&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;_&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;+&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;=&quot; , &quot;'&quot; , &quot;) {*/
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                    //alert(&quot;comma detected1&quot;);
                /*} else {
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                }*/
            });
        }


	.div-container{
		min-width: 700px;
		width: 100%;
	}
	.div-header{
		min-width: 700px;
	}
	.div-body{
		overflow-x: auto; 
		overflow-y: auto; 
		white-space: nowrap;
	}
	.div-data{
		overflow-x: auto; 
		overflow-y: auto; 
		white-space: nowrap;
	}
	.div-footer{
		padding-top: 10px;
		border-bottom: 1px solid #eeeeee;
	}
	.hr-double{
		border-top:3px double #8c8c8c;
		border-bottom:3px double #8c8c8c;
	}
  .hr-double-top{
    border-top:3px double #8c8c8c;
	}
  .hr-double-bottom{
  	border-bottom:3px double #8c8c8c;
	}
	.hr-double-left{
    border-left:3px double #8c8c8c;
	}
  .hr-double-right{
    border-right:3px double #8c8c8c;
	}
	.table-data{
		width: 100%;
		border-collapse: collapse;
		border-color: #c0c0c0;
		background-color: #ffffff;
	}
	.table-data th{
		padding: 10px 6px 10px 6px;
		font-weight: bold;
		text-align: left;
	}
	.table-data td{
		padding: 4px 6px 4px 6px;
		text-align: left;
		border-bottom: 1px solid #c0c0c0;
	}
	.table-data tr:last-child td{
		border-bottom:3px double #8c8c8c;
	}
	.table-data tbody tr:hover{
		cursor: pointer;
		background-color:#f5f5f5;
	}
  .nohover-color:hover {
		cursor: pointer!important;
    background-color:#FFFFFF!important;
	}
	.value-modified{
    background-color: #b4eeb4!important;
  }
  .l_frm{width: 180px; clear: left; float: left;margin-bottom: 2px;text-align: right; margin-right: 2px;}
    .r_frm{float: left;margin-bottom: 2px;}
    .r_frm input,.r_frm select {
        border-radius: 2px; 
        -moz-border-radius: 2px; 
        -webkit-border-radius: 2px; 
        border: 1px solid #bbb;
    }
    .column {
	  float: left;
	  padding: 1px;
	  /*height: 200px; /* Should be removed. Only for demonstration */
	}

	/* Clear floats after the columns */
	.row:after {
	  content: &quot;&quot;;
	  display: table;
	  clear: both;
	}

	input.button-accept {
	  width: 100%;
	  height:30px;
	  box-sizing: border-box;
	  border: 2px solid #ccc;
	  border-radius: 4px;
	  font-size: 18px;
	  font-weight: bold;
	  color:white;
	  background-color: #0091FF	;
	  /*background-image: url(../../images/app_form_edit.png);*/ 
	  background-image: url(../../images/accept.png);
	  background-position: 0px 0px; 
	  background-repeat: no-repeat;
	  padding: 0px 20px 12px 40px;
	}

	input.button-close {
	  width: 100%;
	  height:30px;
	  box-sizing: border-box;
	  border: 2px solid #ccc;
	  border-radius: 4px;
	  font-size: 18px;
	  font-weight: bold;
	  color:white;
	  background-color: #0091FF	;
	  background-image: url(../../images/cancel.png);
	  /*background-size: 20px;*/
	  background-position: 0px 0px; 
	  background-repeat: no-repeat;
	  padding: 0px 20px 12px 40px;
	}

	input.button-proses {
	  width: 100%;
	  height:30px;
	  box-sizing: border-box;
	  border: 2px solid #ccc;
	  border-radius: 4px;
	  font-size: 18px;
	  font-weight: bold;
	  color:white;
	  background-color: #0091FF	;
	  background-image: url(../../images/proses.png);
	  background-position: 0px 0px; 
	  background-repeat: no-repeat;
	  padding: 0px 20px 12px 40px;
	}

	input.button-verif {
	  width: 100%;
	  height:30px;
	  box-sizing: border-box;
	  border: 2px solid #ccc;
	  border-radius: 4px;
	  font-size: 18px;
	  font-weight: bold;
	  color:white;
	  background-color: #0091FF	;
	  background-image: url(../../images/open.gif);
	  background-position: 0px 0px; 
	  background-repeat: no-repeat;
	  padding: 0px 20px 12px 40px;
	}

	input.button-back {
	  width: 100%;
	  height:30px;
	  box-sizing: border-box;
	  border: 2px solid #ccc;
	  border-radius: 4px;
	  font-size: 18px;
	  font-weight: bold;
	  color:white;
	  background-color: #0091FF	;
	  background-image: url(../../images/application_view_columns.png); 
	  background-position: 0px 0px; 
	  background-repeat: no-repeat;
	  padding: 0px 20px 12px 40px;
	}




	AS1031 - CMS FAQ Kategori dan Sub Kategori [new] 


    
    
        
            
                
                
                
                    
                    
                    	
                        
                            
                                
                                    
                                    
                                        
                                            
                                            Detil Data					
                                             
                                             
                                             
                                            
                                                
                                                
                                                                                                     
                                                         Data Ke-1
                                                                                                                      
                                                                    Tipe * :
                                                                    
                                                                            
                                                                            SUB KATEGORI
                                                                    SUB KATEGORI
                                                              
                                                             
                                                             
                                                                    Nama Kategori * :
                                                                    
                                                                        FKAT001 - UMUMFKAT002 - PENERIMA UPAHFKAT003 - BUKAN PENERIMA UPAHFKAT004 - JAMINAN HARI TUA (JHT)FKAT005 - JAMINAN KECELAKAAN KERJA (JKK)FKAT006 - JAMINAN KEMATIAN (JKM)FKAT007 - JAMINAN PENSIUN (JP)FKAT008 - JAMINAN KEHILANGAN PEKERJAAN (JKP)FKAT009 - JASA KONSTRUKSIFKAT010 - LAYANAN SYARIAHFKAT011 - MANFAAT LAYANAN TAMBAHAN (MLT)FKAT012 - KANAL LAYANANFKAT013 - E-CHANNELFKAT014 - PEKERJA MIGRAN INDONESIA (PMI)FKAT015 - UJI KATALON 202412                                                                    FKAT015 - UJI KATALON 202412
                                                             		
                                                             
                                                             
                                                                    Nama Sub Kategori * :
                                                                    
                                                             		
                                                              
                                                             
                                                                    No Urut * :
                                                                    
                                                                            123456789101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899100                                                                    1
                                                              
                                                              
                                                             
                                                                    Status * :
                                                                    
                                                                            TAMBAH DATA SUB KATEGORI
                                                                            
                                                                    TAMBAH DATA SUB KATEGORI
                                                             
                                                                                                              
                                                                                                
                                            
                                        
                                    	
                                    
                                    
                                    
                                    
                                    
                                    
                                             

                                                                                		
                                            											
                                                    
                                                    
                                                    
                                            
                                     	
                                    
                                    
                                    
                                            Keterangan:
                                             Klik tombol TUTUP untuk kembali ke halaman utama
                                    
                                    
                                							
                            		
                        
                        
                        
                    
                	
            
        
    



 

  
    Loading...
  


/html[1]&quot;) or . = concat(&quot;
  
   - 
  
  
	
    

$(document).ready(function(){	
	setTimeout(function(){
		$(&quot; , &quot;'&quot; , &quot;#loading&quot; , &quot;'&quot; , &quot;).hide();
		$(&quot; , &quot;'&quot; , &quot;#loading-mask&quot; , &quot;'&quot; , &quot;).hide();
	}, 250);
});
function alertError(msg){
	window.parent.Ext.MessageBox.show({
	   title: &quot; , &quot;'&quot; , &quot;Perhatian&quot; , &quot;'&quot; , &quot;,
	   msg: msg,
	   buttons: window.parent.Ext.MessageBox.OK,
	   icon: &quot; , &quot;'&quot; , &quot;x-message-box-error&quot; , &quot;'&quot; , &quot;
   });	
}
function alert(msg){
	window.parent.Ext.MessageBox.alert(&quot; , &quot;'&quot; , &quot;Perhatian&quot; , &quot;'&quot; , &quot;, msg);
}
function alertMsg(msg){
	window.parent.Ext.MessageBox.alert(&quot; , &quot;'&quot; , &quot;Informasi&quot; , &quot;'&quot; , &quot;, msg);
}
window.notificationCountAlertNew = window.notificationCountAlertNew || 0;
function showAutoCloseAlert(msg, typeAlert = &quot; , &quot;'&quot; , &quot;info&quot; , &quot;'&quot; , &quot;, title = &quot;Perhatian&quot;, countClose = 4000) {
	console.log(window.notificationCountAlertNew);
	let baseColorAlert = &quot; , &quot;'&quot; , &quot;#4CAF50&quot; , &quot;'&quot; , &quot;
	let backGroundColorAlert = &quot; , &quot;'&quot; , &quot;#f0f8ff&quot; , &quot;'&quot; , &quot;
	let textColroAlert = &quot; , &quot;'&quot; , &quot;#000&quot; , &quot;'&quot; , &quot;
	switch (typeAlert) {
		case &quot; , &quot;'&quot; , &quot;info&quot; , &quot;'&quot; , &quot;:
			baseColorAlert = &quot; , &quot;'&quot; , &quot;#3A87AD&quot; , &quot;'&quot; , &quot;
			backGroundColorAlert = &quot; , &quot;'&quot; , &quot;#f0f8ff&quot; , &quot;'&quot; , &quot;
			textColroAlert = &quot; , &quot;'&quot; , &quot;#000&quot; , &quot;'&quot; , &quot;
			break;
		case &quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;:
			baseColorAlert = &quot; , &quot;'&quot; , &quot;#4CAF50&quot; , &quot;'&quot; , &quot;
			backGroundColorAlert = &quot; , &quot;'&quot; , &quot;#f0f8ff&quot; , &quot;'&quot; , &quot;
			textColroAlert = &quot; , &quot;'&quot; , &quot;#000&quot; , &quot;'&quot; , &quot;
			break;
		case &quot; , &quot;'&quot; , &quot;warning&quot; , &quot;'&quot; , &quot;:
			baseColorAlert = &quot; , &quot;'&quot; , &quot;#C09853&quot; , &quot;'&quot; , &quot;
			backGroundColorAlert = &quot; , &quot;'&quot; , &quot;#f0f8ff&quot; , &quot;'&quot; , &quot;
			textColroAlert = &quot; , &quot;'&quot; , &quot;#000&quot; , &quot;'&quot; , &quot;
			break;
		case &quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;:
			baseColorAlert = &quot; , &quot;'&quot; , &quot;#B94A48&quot; , &quot;'&quot; , &quot;
			backGroundColorAlert = &quot; , &quot;'&quot; , &quot;#f0f8ff&quot; , &quot;'&quot; , &quot;
			textColroAlert = &quot; , &quot;'&quot; , &quot;#000&quot; , &quot;'&quot; , &quot;
			break;
		default:
			baseColorAlert = &quot; , &quot;'&quot; , &quot;#4CAF50&quot; , &quot;'&quot; , &quot;
			backGroundColorAlert = &quot; , &quot;'&quot; , &quot;#f0f8ff&quot; , &quot;'&quot; , &quot;
			textColroAlert = &quot; , &quot;'&quot; , &quot;#000&quot; , &quot;'&quot; , &quot;
			break;
	}
	var baseY = 20;
	var notificationHeight = 110;
	var currentY = baseY + (window.notificationCountAlertNew * notificationHeight);
	if (window.notificationCountAlertNew+1 &lt; 6) {
		var win = window.parent.Ext.create(&quot; , &quot;'&quot; , &quot;Ext.window.Window&quot; , &quot;'&quot; , &quot;, {
			title: title,
			html: msg,
			width: 450,
			height: 100,
			closable: true,
			bodyStyle: `padding: 10px; background-color: ${backGroundColorAlert}; color: #000; border-radius: 8px;`,
			autoShow: true,
			modal: false,
			shadow: true,
			resizable: false,
			draggable: false,
			x: (window.parent.Ext.getBody().getViewSize().width - 450) / 2,
			y: currentY,
			style: {
				borderColor: `${baseColorAlert} !important`,
				borderStyle: &quot; , &quot;'&quot; , &quot;solid&quot; , &quot;'&quot; , &quot;,
				borderWidth: &quot; , &quot;'&quot; , &quot;2px&quot; , &quot;'&quot; , &quot;,
				borderRadius: &quot; , &quot;'&quot; , &quot;8px&quot; , &quot;'&quot; , &quot;
			},
			header: {
				style: {
					borderColor: `${baseColorAlert} !important`,
					backgroundColor: `${baseColorAlert}`,
					color: &quot; , &quot;'&quot; , &quot;#fff&quot; , &quot;'&quot; , &quot;,
					borderRadius: &quot; , &quot;'&quot; , &quot;8px 8px 0 0&quot; , &quot;'&quot; , &quot;
				},
			},
			listeners: {
				afterrender: function(win) {
					// Remove the background color of the existing close button
					var closeButton = win.header.el.down(&quot; , &quot;'&quot; , &quot;.x-tool-close&quot; , &quot;'&quot; , &quot;);
					if (closeButton) {
						closeButton.setStyle({
							backgroundColor: &quot; , &quot;'&quot; , &quot;transparent&quot; , &quot;'&quot; , &quot;, // Remove background color
							border: &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;, // Remove border if any
							color: &quot; , &quot;'&quot; , &quot;#fff&quot; , &quot;'&quot; , &quot; // Set icon color
						});
					}

					// Close the window after a custom delay
					window.parent.Ext.defer(function() {
						win.close();
						if (window.notificationCountAlertNew > 0) {
							window.notificationCountAlertNew--; // Decrement the counter only if it&quot; , &quot;'&quot; , &quot;s greater than 0
						}
					}, countClose); // Time in milliseconds (4 seconds)
				},
				close: function(win) {
					// Trigger when the X button is clicked
					if (window.notificationCountAlertNew > 0) {
						window.notificationCountAlertNew--; // Decrement the counter only if it&quot; , &quot;'&quot; , &quot;s greater than 0
					}
				}
			}
		});

		window.notificationCountAlertNew++;
	}
}

#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}






    .saved_message_success {
        font-weight:bold;font-size:12px;color:black;padding-bottom:30px;padding-top:30px;
    }
    .saved_message_failed {
        font-weight:bold;font-size:12px;color:red;padding-bottom:30px;padding-top:30px;
    }

    div.DialogMask
    {
        padding: 10px;
        position: fixed;
        top: 0;
        right: 0;
        bottom: 0;
        left: 0;
        z-index: 50;
        background-color: #606060;
        filter: progid:DXImageTransform.Microsoft.Alpha(Opacity=50);
        -moz-opacity: .5;
        opacity: .5;
    }





  



    //logger.disableLogger();
    logger.enableLogger();
	$(document).ready(function(){
            $(&quot;input[type=text]&quot;).keyup(function(){
                    //$(this).val($(this).val().toUpperCase());
            });
            
            $(&quot; , &quot;'&quot; , &quot;.kategori&quot; , &quot;'&quot; , &quot;).keypress(function (e) {
                console.log(&quot;keypress button: e.key=&quot;+e.key+&quot; - e.code=&quot;+e.code+&quot; - e.which=&quot;+e.which);
                /*if (e.which === 44 || e.key === &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&lt;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;>&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;?&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&quot;&quot; , &quot;'&quot; , &quot; || e.key === &quot;&quot; , &quot;'&quot; , &quot;&quot; || e.key === &quot; , &quot;'&quot; , &quot;{&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;[&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;}&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;|&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;\\&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;`&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;~&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;!&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;@&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;$&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;%&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;^&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;*&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;(&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;)&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;_&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;+&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;=&quot; , &quot;'&quot; , &quot;) {*/
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                    //alert(&quot;comma detected1&quot;);
                /*} else {
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replaceAll(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                }*/
            }).keyup(function (e) {
                /*if (e.which === 44 || e.key === &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&lt;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;>&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;?&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&quot;&quot; , &quot;'&quot; , &quot; || e.key === &quot;&quot; , &quot;'&quot; , &quot;&quot; || e.key === &quot; , &quot;'&quot; , &quot;{&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;[&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;}&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;|&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;\\&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;`&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;~&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;!&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;@&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;$&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;%&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;^&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;*&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;(&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;)&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;_&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;+&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;=&quot; , &quot;'&quot; , &quot;) {*/
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                    //alert(&quot;comma detected1&quot;);
                /*} else {
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replaceAll(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                }*/
            });
            
            $(&quot; , &quot;'&quot; , &quot;.subkategori&quot; , &quot;'&quot; , &quot;).keypress(function (e) {
                console.log(&quot;keypress button: e.key=&quot;+e.key+&quot; - e.code=&quot;+e.code+&quot; - e.which=&quot;+e.which);
                /*if (e.which === 44 || e.key === &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&lt;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;>&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;?&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&quot;&quot; , &quot;'&quot; , &quot; || e.key === &quot;&quot; , &quot;'&quot; , &quot;&quot; || e.key === &quot; , &quot;'&quot; , &quot;{&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;[&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;}&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;|&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;\\&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;`&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;~&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;!&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;@&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;$&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;%&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;^&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;*&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;(&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;)&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;_&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;+&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;=&quot; , &quot;'&quot; , &quot;) {*/
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                    //alert(&quot;comma detected1&quot;);
                /*} else {
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                }*/
            }).keyup(function (e) {
                /*if (e.which === 44 || e.key === &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&lt;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;>&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;?&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&quot;&quot; , &quot;'&quot; , &quot; || e.key === &quot;&quot; , &quot;'&quot; , &quot;&quot; || e.key === &quot; , &quot;'&quot; , &quot;{&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;[&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;}&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;|&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;\\&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;`&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;~&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;!&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;@&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;$&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;%&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;^&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;*&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;(&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;)&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;_&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;+&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;=&quot; , &quot;'&quot; , &quot;) {*/
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                    //alert(&quot;comma detected1&quot;);
                /*} else {
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                }*/
            });

            $(&quot; , &quot;'&quot; , &quot;select.tipe_input&quot; , &quot;'&quot; , &quot;).select2();
            $(&quot; , &quot;'&quot; , &quot;select#kategori&quot; , &quot;'&quot; , &quot;).select2();
            $(&quot; , &quot;'&quot; , &quot;select.kategori&quot; , &quot;'&quot; , &quot;).select2();
            $(&quot; , &quot;'&quot; , &quot;select#tipe&quot; , &quot;'&quot; , &quot;).select2();
            $(&quot; , &quot;'&quot; , &quot;select.tipe&quot; , &quot;'&quot; , &quot;).select2();
            $(&quot; , &quot;'&quot; , &quot;select.seq&quot; , &quot;'&quot; , &quot;).select2();
            $(&quot; , &quot;'&quot; , &quot;select.status&quot; , &quot;'&quot; , &quot;).select2();
            $(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;).select2();

            $(window).bind(&quot;resize&quot;, function(){
                resize();
            });
            resize();
            // filter();
            /** list checkbox npp */
            window.list_npp = [];
	});
	var asyncPreloadStart;

	function asyncPreloadX(state){
            if (state == true) {
                asyncPreloadStart = setInterval(function() {
                    $(&quot; , &quot;'&quot; , &quot;#loading&quot; , &quot;'&quot; , &quot;).show();
                    $(&quot; , &quot;'&quot; , &quot;#loading-mask&quot; , &quot;'&quot; , &quot;).show();
                }, 50);
            } else {
                $(&quot; , &quot;'&quot; , &quot;#loading&quot; , &quot;'&quot; , &quot;).hide();
                $(&quot; , &quot;'&quot; , &quot;#loading-mask&quot; , &quot;'&quot; , &quot;).hide();
                clearInterval(asyncPreloadStart);
            }
	}
        
        function preloadMask(param_true_false) {
            if (param_true_false==true) {
                if ($(&quot; , &quot;'&quot; , &quot;.DialogMask&quot; , &quot;'&quot; , &quot;).length == 0) {
                    $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;DialogMask&quot; style=&quot;z-index:20000;background-color:black;opacity:0.5;width:100%;height:100vh;display:flex;justify-content:center;align-items:center;&quot;>&quot; , &quot;'&quot; , &quot;+
                                        &quot; , &quot;'&quot; , &quot;&lt;img style=&quot;width:40px;height:auto;&quot; src=&quot;http://172.28.108.46:5454/smile/images/loading.gif&quot; />&quot; , &quot;'&quot; , &quot;+
                                        &quot; , &quot;'&quot; , &quot;&lt;span style=&quot;display:inline-block;position:fixed;bottom:130px;right:0px;left:0px;text-align:center;color:white;font-weight:bold;font-size:12px;font-style:italic;padding-left:10px;&quot;>in progress...&lt;/span>&quot; , &quot;'&quot; , &quot;+
                                     &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;);
                }
                $(&quot; , &quot;'&quot; , &quot;.DialogMask&quot; , &quot;'&quot; , &quot;).show();
            } else {
                $(&quot; , &quot;'&quot; , &quot;.DialogMask&quot; , &quot;'&quot; , &quot;).hide();
            }
        }
	
	function getValue(val){
            return val == null || val == undefined ? &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; : val;
	}

	function search_by_changed(){
            $(&quot;#search_txt&quot;).val(&quot;&quot;);
            $(&quot;#search_txt2&quot;).val(&quot;&quot;);
	}

	function isNumber(evt) {
            evt = (evt) ? evt : window.event;
            var charCode = (evt.which) ? evt.which : evt.keyCode;
            if (charCode > 31 &amp;&amp; (charCode &lt; 48 || charCode > 57)) {
                return false;
            }
            return true;
	}
	
	function validateDigit(evt) {
            var theEvent = evt || window.event;
            var key = theEvent.keyCode || theEvent.which;
            key = String.fromCharCode(key);
            var regex = /[0-9]|\./;
            if (!regex.test(key)) {
                theEvent.returnValue = false;
                if (theEvent.preventDefault)
                    theEvent.preventDefault();
            }
        }
	
	function Comma(Num) { //function to add commas to textboxes
            Num += &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            Num = Num.replace(&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            Num = Num.replace(&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            Num = Num.replace(&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            Num = Num.replace(&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            Num = Num.replace(&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            Num = Num.replace(&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            x = Num.split(&quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;);
            x1 = x[0];
            x2 = x.length > 1 ? &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot; + x[1] : &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            var rgx = /(\d+)(\d{3})/;
            while (rgx.test(x1))
                            x1 = x1.replace(rgx, &quot; , &quot;'&quot; , &quot;$1&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;$2&quot; , &quot;'&quot; , &quot;);
            return x1 + x2;
	}

	function isNumberKey(evt) {
            var theEvent = evt || window.event;
            var key = theEvent.keyCode || theEvent.which;
            key = String.fromCharCode(key);
            if (key.length == 0)
                    return;
            var regex = /^[0-9\b]+$/;
            if (!regex.test(key)) {
                    theEvent.returnValue = false;
                    if (theEvent.preventDefault)
                            theEvent.preventDefault();
            }
	}

	function resize(){
            $(&quot;#div_container&quot;).width($(&quot;#div_dummy&quot;).width());

            $(&quot;#div_header&quot;).width($(&quot;#div_dummy&quot;).width());
            $(&quot;#div_body&quot;).width($(&quot;#div_dummy&quot;).width());
            $(&quot;#div_footer&quot;).width($(&quot;#div_dummy&quot;).width());

            $(&quot;#div_filter&quot;).width(0);
            $(&quot;#div_data&quot;).width(0);
            $(&quot;#div_page&quot;).width(0);
            $(&quot;#div_footer&quot;).width(0);

            $(&quot;#div_filter&quot;).width($(&quot;#div_dummy_data&quot;).width());
            $(&quot;#div_data&quot;).width($(&quot;#div_dummy_data&quot;).width());
            $(&quot;#div_page&quot;).width($(&quot;#div_dummy_data&quot;).width());
            $(&quot;#div_footer&quot;).width($(&quot;#div_dummy_data&quot;).width());

            $(&quot;#div_container&quot;).css(&quot; , &quot;'&quot; , &quot;max-height&quot; , &quot;'&quot; , &quot;, $(window).height());

            let margin_height = 40;
            let filter_height = $(&quot; , &quot;'&quot; , &quot;#div_body&quot; , &quot;'&quot; , &quot;).height() - $(&quot; , &quot;'&quot; , &quot;#div_data&quot; , &quot;'&quot; , &quot;).height() + margin_height;
            $(&quot; , &quot;'&quot; , &quot;#div_data&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;max-height&quot; , &quot;'&quot; , &quot;, $(window).height() - $(&quot; , &quot;'&quot; , &quot;#div_header&quot; , &quot;'&quot; , &quot;).height() - $(&quot; , &quot;'&quot; , &quot;#div_page&quot; , &quot;'&quot; , &quot;).height() - $(&quot; , &quot;'&quot; , &quot;#div_footer&quot; , &quot;'&quot; , &quot;).height() - filter_height);
	}
	
	function confirmation(title, msg, fnYes, fnNo) {
            window.parent.Ext.Msg.show({
                title: title,
                msg: msg,
                buttons: window.parent.Ext.Msg.YESNO,
                icon: window.parent.Ext.Msg.QUESTION,
                fn: function(btn) {
                    if (btn === &quot; , &quot;'&quot; , &quot;yes&quot; , &quot;'&quot; , &quot;) {
                        fnYes();
                    } else {
                        fnNo();
                    }
                }
            });
	}

	function showFormReload(mypage, myname, w, h, scroll) {
            var openwin = window.parent.Ext.create(&quot; , &quot;'&quot; , &quot;Ext.window.Window&quot; , &quot;'&quot; , &quot;, {
                title: myname,
                collapsible: true,
                animCollapse: true,
                maximizable: true,
                closable: true,
                width: w,
                height: h,
                minWidth: w,
                minHeight: h,
                layout: &quot; , &quot;'&quot; , &quot;fit&quot; , &quot;'&quot; , &quot;,
                modal: true,
                html: &quot; , &quot;'&quot; , &quot;&lt;iframe src=&quot;&quot; , &quot;'&quot; , &quot; + mypage + &quot; , &quot;'&quot; , &quot;&quot;  frameborder=&quot;0&quot; style=&quot;border:0; height:100%; width:100%; overflow-y:hidden; overflow-x:hidden; overflow:hidden;&quot; scrolling=&quot;no&quot;>&lt;/iframe>&quot; , &quot;'&quot; , &quot;,
                listeners: {
                    close: function () {
                        resubmit();
                    },
                        destroy: function (wnd, eOpts) {
                    }
                }
            });
            openwin.show();
            return openwin;
	}

	function showFormRefilter(mypage, myname, w, h, scroll) {
            var openwin = window.parent.Ext.create(&quot; , &quot;'&quot; , &quot;Ext.window.Window&quot; , &quot;'&quot; , &quot;, {
                title: myname,
                collapsible: true,
                animCollapse: true,
                maximizable: true,
                closable: true,
                width: w,
                height: h,
                minWidth: w,
                minHeight: h,
                layout: &quot; , &quot;'&quot; , &quot;fit&quot; , &quot;'&quot; , &quot;,
                modal: true,
                html: &quot; , &quot;'&quot; , &quot;&lt;iframe src=&quot;&quot; , &quot;'&quot; , &quot; + mypage + &quot; , &quot;'&quot; , &quot;&quot;  frameborder=&quot;0&quot; style=&quot;border:0; height:100%; width:100%; overflow-y:hidden; overflow-x:hidden; overflow:hidden;&quot; scrolling=&quot;no&quot;>&lt;/iframe>&quot; , &quot;'&quot; , &quot;,
                listeners: {
                    close: function () {
                        filter();
                    },
                        destroy: function (wnd, eOpts) {
                    }
                }
            });
            openwin.show();
            return openwin;
	}

	function showForm(mypage, myname, w, h, scroll) {
            var openwin = window.parent.Ext.create(&quot; , &quot;'&quot; , &quot;Ext.window.Window&quot; , &quot;'&quot; , &quot;, {
                title: myname,
                collapsible: true,
                animCollapse: true,
                maximizable: true,
                closable: true,
                width: w,
                height: h,
                minWidth: w,
                minHeight: h,
                layout: &quot; , &quot;'&quot; , &quot;fit&quot; , &quot;'&quot; , &quot;,
                modal: true,
                html: &quot; , &quot;'&quot; , &quot;&lt;iframe src=&quot;&quot; , &quot;'&quot; , &quot; + mypage + &quot; , &quot;'&quot; , &quot;&quot;  frameborder=&quot;0&quot; style=&quot;border:0; height:100%; width:100%; overflow-y:hidden; overflow-x:hidden; overflow:hidden;&quot; scrolling=&quot;no&quot;>&lt;/iframe>&quot; , &quot;'&quot; , &quot;,
                listeners: {
                    close: function () {
                    },
                        destroy: function (wnd, eOpts) {
                    }
                }
            });
            openwin.show();
            return openwin;
	}

	function getBase64(file) {
            return new Promise((resolve, reject) => {
		const reader = new FileReader();
		reader.readAsDataURL(file);
		reader.onload = () => resolve(reader.result);
		reader.onerror = error => reject(error);
            });
	}
        
        function enableSelect2EditFailed(data_ke) {
            console.log(&quot;enableSelect2EditFailed: &quot;+data_ke);
            $(&quot;#tipe_input&quot;+data_ke+&quot;, #kategori&quot;+data_ke+&quot;, #seq&quot;+data_ke+&quot;, #status&quot;+data_ke).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;);
            $(&quot;#tipe_input&quot;+data_ke+&quot;, #kategori&quot;+data_ke+&quot;, #seq&quot;+data_ke+&quot;, #status&quot;+data_ke).select2({allowClear: true});
            $(&quot;#tipe_input&quot;+data_ke+&quot;, #kategori&quot;+data_ke+&quot;, #seq&quot;+data_ke+&quot;, #status&quot;+data_ke).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
            //$(&quot;#tipe_input&quot;+data_ke+&quot; select:first-of-type, #kategori&quot;+data_ke+&quot; select:first-of-type, #seq&quot;+data_ke+&quot; select:first-of-type, #status&quot;+data_ke+&quot; select:first-of-type&quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
            //$(&quot;#tipe_input&quot;+data_ke+&quot;:first-of-type, #kategori&quot;+data_ke+&quot;:first-of-type, #seq&quot;+data_ke+&quot;:first-of-type, #status&quot;+data_ke+&quot;:first-of-type&quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
            $(&quot;#tipe_input&quot;+data_ke+&quot;, #kategori&quot;+data_ke+&quot;, #seq&quot;+data_ke+&quot;, #status&quot;+data_ke).on(&quot; , &quot;'&quot; , &quot;select2:select&quot; , &quot;'&quot; , &quot;, function(e) {
                $(this).nextAll(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;).first().prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
            });

            /*$(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;).select2();
            $(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;).prop(&quot;disabled&quot;, true);
            $(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;).eq(0).prop(&quot;disabled&quot;, false);
            $(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;select2:select&quot; , &quot;'&quot; , &quot;, function(e) {
              var index = $(this).index(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;);
              $(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;).eq(index + 1).prop(&quot;disabled&quot;, false);
            });*/

            $(&quot;#tipe_input&quot;+data_ke+&quot;, #kategori&quot;+data_ke+&quot;, #seq&quot;+data_ke+&quot;, #status&quot;+data_ke).select2(&quot;enable&quot;, true);
        }
	
	async function save(parameter=&quot;&quot;){
            if (parameter===&quot;new&quot;) {
		confirmation(&quot; , &quot;'&quot; , &quot;Simpan Data&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Yakin untuk simpan data?&quot; , &quot;'&quot; , &quot;, async function(){
                    empty_field = 0;
                    length_field_100 = 0;
                    $(&quot; , &quot;'&quot; , &quot;input[type=text]&quot; , &quot;'&quot; , &quot;).each(function() {
                        if($(this).val() == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                           empty_field += 1;
                           $(this).css(&quot; , &quot;'&quot; , &quot;background-color&quot; , &quot;'&quot; , &quot; , &quot; , &quot;'&quot; , &quot;#f8c2b6&quot; , &quot;'&quot; , &quot;); //FF0000
                        } else {
                            if ($(this).val().length > 100) {
                                length_field_100++;
                                $(this).css(&quot; , &quot;'&quot; , &quot;background-color&quot; , &quot;'&quot; , &quot; , &quot; , &quot;'&quot; , &quot;#f8c2b6&quot; , &quot;'&quot; , &quot;); //FF0000
                            }
                        }
                    });
                    if (empty_field > 0) return alert(&quot; , &quot;'&quot; , &quot;Tidak boleh kosong&quot; , &quot;'&quot; , &quot;);
                    if (length_field_100 > 0) return alert(&quot; , &quot;'&quot; , &quot;Field size tidak boleh melebihi 100 karakter. Length saat ini: &quot; , &quot;'&quot; , &quot;+length_field_100);
                    
                    const form = $(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;)[0]; 
                    let formData = new FormData(form);
                    formData.append(&quot; , &quot;'&quot; , &quot;tipe&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;save_new&quot; , &quot;'&quot; , &quot;);

                    preloadMask(true);
                    $.ajax({
                        type: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;,
                        url: &quot;http://172.28.108.46:5454/smile/mod_ec/ajax/as1031_action.php?&quot;+Math.random(),
                        data: formData ? formData : form.serialize(),
                        contentType : false,
                        processData: false,
                        success: function(datas){
                            //alert(datas);
                            console.log(&quot;datas&quot;,datas);
                            jdata = JSON.parse(datas);
                            console.log(jdata);
                            console.log(&quot;jdata.length: &quot;+Object.keys(jdata).length);
                            if (Object.keys(jdata).length==1) {
                                if (jdata[0].ret == 1){
                                    alert(jdata[0].msg);
                                    //location.replace(&quot; , &quot;'&quot; , &quot;http://&lt; ?=$HTTP_HOST;?>/mod_ec/form/as1031.php?former_kategori_subkategori=&lt; ?=$former_kategori_subkategori?>&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&lt; ?=$former_search_txt?>&quot;+&quot;&amp;former_search_txt2=&lt; ?=$former_search_txt2?>&quot;+&quot;&amp;former_search_txt3=&lt; ?=$former_search_txt3?>&quot;);
                                    //location.replace(&quot; , &quot;'&quot; , &quot;http://172.28.108.46:5454/smile/mod_ec/form/as1031.php?former_kategori_subkategori=Y&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&quot;+&quot;&amp;former_search_txt2=&quot;+&quot;&amp;sess=div-container-table&quot;+&quot;&amp;selected_data=&quot;+&quot;&amp;status_crud=sukses&quot;);
                                    location.replace(&quot; , &quot;'&quot; , &quot;http://172.28.108.46:5454/smile/mod_ec/form/as1031.php?former_kategori_subkategori=Y&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&quot;+&quot;&amp;former_search_txt2=&quot;);
                                } else {
                                    alert(jdata[0].msg);
                                }
                            } else {
                                $(&quot;#saved_message&quot;).html(&quot;&quot;);
                                $(&quot;#saved_message&quot;).show();
                                var return_message = &quot;&quot;;
                                var failed = &quot;&quot;;
                                $.each(jdata, function(idx, val) {
                                    return_message = return_message + &quot;Data ke-&quot; + val.ke + &quot;: &quot; + val.msg + &quot;\n\r&lt;br/>&quot;;
                                    if (val.ret==&quot;1&quot;) {
                                        $(&quot;#saved_message&quot;).append(&quot;&lt;span class=&quot; , &quot;'&quot; , &quot;saved_message_success&quot; , &quot;'&quot; , &quot;>idx ke: &quot;+idx+&quot;, val ke: &quot;+val.ke+&quot;, val ret: &quot;+val.ret+&quot;, val msg: &quot;+val.msg+&quot;, val no urut: &quot;+val.noUrut+&quot;, val input: &quot;+val.input+&quot;, val status: &quot;+val.status+&quot;&lt;span>&lt;br/>&lt;br/>                        \n\n\n\n&quot;);
                                        $(&quot;#saved_message&quot;).append(&quot;\n\r&quot;);
                                        $(&quot;#saved_message&quot;).append(&quot;\n\r&quot;);
                                        
                                        div_color = &quot;&lt;div style=&quot; , &quot;'&quot; , &quot;z-index:2;background: linear-gradient(159deg, rgba(173,223,173,1) 0%, rgba(50,205,50,1) 100%);opacity:0.3;position:absolute;border-radius:10px;left:0px;top:0px;right:0px;bottom:0px;&quot; , &quot;'&quot; , &quot;>&lt;/div>&quot;;
                                        if (idx==0) {
                                            $(&quot;.konten_form0&quot;).append(div_color);
                                            $(&quot;.konten_form0 input[type=text]&quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
                                            $(&quot;.konten_form0 input[type=text]&quot;).attr(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;);
                                            $(&quot;.konten_form0 select&quot;).select2(&quot;enable&quot;, false);
                                        } else {
                                            $(&quot;.konten_form&quot;+(idx+1)).append(div_color);
                                            $(&quot;.konten_form&quot;+(idx+1)+&quot; input[type=text]&quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
                                            $(&quot;.konten_form&quot;+(idx+1)+&quot; input[type=text]&quot;).attr(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;);
                                            $(&quot;.konten_form&quot;+(idx+1)+&quot; select&quot;).select2(&quot;enable&quot;, false);
                                        }
                                    } else {
                                        $(&quot;#saved_message&quot;).append(&quot;&lt;span class=&quot; , &quot;'&quot; , &quot;saved_message_failed&quot; , &quot;'&quot; , &quot;>idx ke: &quot;+idx+&quot;, val ke: &quot;+val.ke+&quot;, val ret: &quot;+val.ret+&quot;, val msg: &quot;+val.msg+&quot;, val no urut: &quot;+val.noUrut+&quot;, val input: &quot;+val.input+&quot;, val status: &quot;+val.status+&quot;&lt;span>&lt;br/>&lt;br/>                        \n\n\n\n&quot;);
                                        $(&quot;#saved_message&quot;).append(&quot;\n\r&quot;);
                                        $(&quot;#saved_message&quot;).append(&quot;\n\r&quot;);
                                        failed += failed + &quot;Data ke-&quot; + val.ke + &quot;: &quot; + val.msg + &quot;\n\r&lt;br/>&quot;;
                                        
                                        if (idx==0) {
                                            $(&quot;.konten_form0 input[type=text]&quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
                                            $(&quot;.konten_form0 input[type=text]&quot;).removeAttr(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;);
                                            //$(&quot;.konten_form0 select&quot;).select2(&quot;enable&quot;, true);
                                        } else {
                                            $(&quot;.konten_form&quot;+(idx+1)+&quot; input[type=text]&quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
                                            $(&quot;.konten_form&quot;+(idx+1)+&quot; input[type=text]&quot;).removeAttr(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;);
                                            //$(&quot;.konten_form&quot;+(idx+1)+&quot; select&quot;).select2(&quot;enable&quot;, true);
                                        }
                                    }
                                });
                                alert(return_message);
                                if (failed==&quot;&quot;) {
                                    //location.replace(&quot; , &quot;'&quot; , &quot;http://&lt; ?=$HTTP_HOST;?>/mod_ec/form/as1031.php?former_kategori_subkategori=&lt; ?=$former_kategori_subkategori?>&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&lt; ?=$former_search_txt?>&quot;+&quot;&amp;former_search_txt2=&lt; ?=$former_search_txt2?>&quot;+&quot;&amp;former_search_txt3=&lt; ?=$former_search_txt3?>&quot;);
                                    //location.replace(&quot; , &quot;'&quot; , &quot;http://&lt; ?=$HTTP_HOST;?>/mod_ec/form/as1031.php?former_kategori_subkategori=&lt; ?=$former_kategori_subkategori?>&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&lt; ?=$former_search_txt?>&quot;+&quot;&amp;former_search_txt2=&lt; ?=$former_search_txt2?>&quot;+&quot;&amp;sess=&lt; ?=$former_sess_table?>&quot;+&quot;&amp;selected_data=&lt; ?=$former_selected_data?>&quot;+&quot;&amp;status_crud=sukses&quot;);
                                    location.replace(&quot; , &quot;'&quot; , &quot;http://172.28.108.46:5454/smile/mod_ec/form/as1031.php?former_kategori_subkategori=Y&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&quot;+&quot;&amp;former_search_txt2=&quot;);
                                }
                            }
                            
//                            if (jdata.ret == 1){
//                                alert(jdata.msg);
//                                //location.replace(&quot; , &quot;'&quot; , &quot;http://&lt; ?= $HTTP_HOST; ?>/mod_ec/form/as1031.php?former_kategori_subkategori=&lt; ?=$former_kategori_subkategori?>&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&lt; ?=$former_search_txt?>&quot;+&quot;&amp;former_search_txt2=&lt; ?=$former_search_txt2?>&quot;+&quot;&amp;former_search_txt3=&lt; ?=$former_search_txt3?>&quot;);
//                            } else {
//                                alert(jdata.msg);
//                            }
                            preloadMask(false);
                        },
                        complete: function(){
                            preloadMask(false);
                        },
                        error: function(){
                            alert(&quot;Terjadi kesalahan, coba beberapa saat lagi!&quot;);
                            preloadMask(false);
                        }
                    });
		}, function(){});
            } else if (parameter===&quot;edit&quot;) {
		confirmation(&quot; , &quot;'&quot; , &quot;Edit Data&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Yakin untuk edit data?&quot; , &quot;'&quot; , &quot;, async function(){
                    empty_field = 0;
                    length_field_100 = 0;
                    $(&quot; , &quot;'&quot; , &quot;input[type=text]&quot; , &quot;'&quot; , &quot;).each(function() {
                        if($(this).val() == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                           empty_field += 1;
                           $(this).css(&quot; , &quot;'&quot; , &quot;background-color&quot; , &quot;'&quot; , &quot; , &quot; , &quot;'&quot; , &quot;#f8c2b6&quot; , &quot;'&quot; , &quot;); //FF0000
                        } else {
                            if ($(this).val().length > 100) {
                                length_field_100++;
                                $(this).css(&quot; , &quot;'&quot; , &quot;background-color&quot; , &quot;'&quot; , &quot; , &quot; , &quot;'&quot; , &quot;#f8c2b6&quot; , &quot;'&quot; , &quot;); //FF0000
                            }
                        }
                    });
                    if (empty_field > 0) return alert(&quot; , &quot;'&quot; , &quot;Tidak boleh kosong&quot; , &quot;'&quot; , &quot;);
                    if (length_field_100 > 0) return alert(&quot; , &quot;'&quot; , &quot;Field size tidak boleh melebihi 100 karakter. Length saat ini: &quot; , &quot;'&quot; , &quot;+length_field_100);
                    
                    const form = $(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;)[0]; 
                    let formData = new FormData(form);
                    formData.append(&quot; , &quot;'&quot; , &quot;tipe&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;save_edit&quot; , &quot;'&quot; , &quot;);

                    preloadMask(true);
                    $.ajax({
                        type: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;,
                        url: &quot;http://172.28.108.46:5454/smile/mod_ec/ajax/as1031_action.php?&quot;+Math.random(),
                        data: formData ? formData : form.serialize(),
                        contentType : false,
                        processData: false,
                        success: function(datas){
                            //alert(datas);
                            console.log(&quot;datas&quot;,datas);
                            sessionStorage.setItem(&quot;kategori_subkategori_save_edit&quot;, datas);
                            console.log(&quot;datas kategori_subkategori_save_edit defined&quot;,datas);
                            jdata = JSON.parse(datas);
                            console.log(jdata);
                            console.log(&quot;jdata.length: &quot;+Object.keys(jdata).length);
                            if (Object.keys(jdata).length==1) {
                                if (jdata[0].ret == 1){
                                    alert(jdata[0].msg);
                                    //location.replace(&quot; , &quot;'&quot; , &quot;http://&lt; ?=$HTTP_HOST;?>/mod_ec/form/as1031.php?former_kategori_subkategori=&lt; ?=$former_kategori_subkategori?>&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&lt; ?=$former_search_txt?>&quot;+&quot;&amp;former_search_txt2=&lt; ?=$former_search_txt2?>&quot;+&quot;&amp;former_search_txt3=&lt; ?=$former_search_txt3?>&quot;);
                                    location.replace(&quot; , &quot;'&quot; , &quot;http://172.28.108.46:5454/smile/mod_ec/form/as1031.php?former_kategori_subkategori=Y&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&quot;+&quot;&amp;former_search_txt2=&quot;+&quot;&amp;sess=div-container-table&quot;+&quot;&amp;selected_data=&quot;+&quot;&amp;status_crud=sukses&quot;);
                                } else {
                                    alert(jdata[0].msg);
                                }
                            } else {
                                $(&quot;#saved_message&quot;).html(&quot;&quot;);
                                $(&quot;#saved_message&quot;).show();
                                var return_message = &quot;&quot;;
                                var failed = &quot;&quot;;
                                $.each(jdata, function(idx, val) {
                                    let data_ke = idx + 1;
                                    return_message = return_message + &quot;Data ke-&quot; + val.ke + &quot;: &quot; + val.msg + &quot;\n\r&lt;br/>&quot;;
                                    if (val.ret==&quot;1&quot;) {
                                        console.log(&quot;idx: &quot;+idx);
                                        $(&quot;#saved_message&quot;).append(&quot;&lt;span class=&quot; , &quot;'&quot; , &quot;saved_message_success&quot; , &quot;'&quot; , &quot;>idx ke: &quot;+data_ke+&quot;, val ke: &quot;+val.ke+&quot;, val ret: &quot;+val.ret+&quot;, val msg: &quot;+val.msg+&quot;, val no urut: &quot;+val.noUrut+&quot;, val input: &quot;+val.input+&quot;, val status: &quot;+val.status+&quot;&lt;/span>&lt;br/>&lt;br/>                        \n\n\n\n&quot;);
                                        $(&quot;#saved_message&quot;).append(&quot;\n\r&quot;);
                                        $(&quot;#saved_message&quot;).append(&quot;\n\r&quot;);
                                    
                                        let div_color = &quot;&lt;div style=&quot; , &quot;'&quot; , &quot;z-index:2;background: linear-gradient(159deg, rgba(173,223,173,1) 0%, rgba(50,205,50,1) 100%);opacity:0.3;position:absolute;border-radius:10px;left:0px;top:0px;right:0px;bottom:0px;&quot; , &quot;'&quot; , &quot;>&lt;/div>&quot;;
                                        $(&quot;.konten_form&quot;+val.ke).append(div_color);
                                        $(&quot;.konten_form&quot;+val.ke+&quot; input[type=text], .konten_form&quot;+val.ke+&quot; input[type=hidden]&quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
                                        $(&quot;.konten_form&quot;+val.ke+&quot; input[type=text], .konten_form&quot;+val.ke+&quot; input[type=hidden]&quot;).attr(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;);
                                        $(&quot;.konten_form&quot;+val.ke+&quot; select&quot;).select2(&quot;enable&quot;, false);
                                        console.log(&quot;data_ke: &quot;+data_ke);
                                    } else {
                                        $(&quot;#saved_message&quot;).append(&quot;&lt;span class=&quot; , &quot;'&quot; , &quot;saved_message_failed&quot; , &quot;'&quot; , &quot;>idx ke: &quot;+data_ke+&quot;, val ke: &quot;+val.ke+&quot;, val ret: &quot;+val.ret+&quot;, val msg: &quot;+val.msg+&quot;, val no urut: &quot;+val.noUrut+&quot;, val input: &quot;+val.input+&quot;, val status: &quot;+val.status+&quot;&lt;/span>&lt;br/>&lt;br/>                        \n\n\n\n&quot;);
                                        $(&quot;#saved_message&quot;).append(&quot;\n\r&quot;);
                                        $(&quot;#saved_message&quot;).append(&quot;\n\r&quot;);
                                        //$(&quot;.konten_form&quot;+val.ke).append(&quot;&lt;input type=&quot; , &quot;'&quot; , &quot;button&quot; , &quot;'&quot; , &quot; onclick=&quot; , &quot;'&quot; , &quot;enableSelect2EditFailed(&quot;+val.ke+&quot;);&quot; , &quot;'&quot; , &quot; value=&quot; , &quot;'&quot; , &quot;Enable Select2&quot; , &quot;'&quot; , &quot; />&quot;);
                                        failed += failed + &quot;Data ke-&quot; + val.ke + &quot;: &quot; + val.msg + &quot;\n\r&lt;br/>&quot;;
                                        
                                        $(&quot;.konten_form&quot;+val.ke+&quot; input[type=hidden]&quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
                                        $(&quot;.konten_form&quot;+val.ke+&quot; input[type=hidden]&quot;).removeAttr(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;, false);
                                        $(&quot;.konten_form&quot;+val.ke+&quot; #subkategori&quot;+val.ke).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
                                        $(&quot;.konten_form&quot;+val.ke+&quot; #subkategori&quot;+val.ke).removeAttr(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;, false);
                                        //$(&quot;select&quot;).select2(&quot;enable&quot;, true);
                                        $(&quot;#kode_subkategori&quot;+val.ke).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;); //hidden
                                        $(&quot;#kode_subkategori&quot;+val.ke).prop(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;, false).removeAttr(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;); //hidden
                                    }
                                });
                                alert(return_message);
                                if (failed==&quot;&quot;) {
                                    //location.replace(&quot; , &quot;'&quot; , &quot;http://&lt; ?=$HTTP_HOST;?>/mod_ec/form/as1031.php?former_kategori_subkategori=&lt; ?=$former_kategori_subkategori?>&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&lt; ?=$former_search_txt?>&quot;+&quot;&amp;former_search_txt2=&lt; ?=$former_search_txt2?>&quot;+&quot;&amp;former_search_txt3=&lt; ?=$former_search_txt3?>&quot;);
                                    location.replace(&quot; , &quot;'&quot; , &quot;http://172.28.108.46:5454/smile/mod_ec/form/as1031.php?former_kategori_subkategori=Y&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&quot;+&quot;&amp;former_search_txt2=&quot;+&quot;&amp;sess=div-container-table&quot;+&quot;&amp;selected_data=&quot;+&quot;&amp;status_crud=sukses&quot;);
                                }
                            }
                            preloadMask(false);
                        },
                        complete: function(){
                            preloadMask(false);
                        },
                        error: function(){
                            alert(&quot;Terjadi kesalahan, coba beberapa saat lagi!&quot;);
                            preloadMask(false);
                        }
                    });
		}, function(){});
            }
	} 

	function kembali(){
            //window.location.replace(&quot; , &quot;'&quot; , &quot;http://&lt; ?=$HTTP_HOST; ?>/mod_ec/form/as1031.php?former_kategori_subkategori=&lt; ?=$former_kategori_subkategori?>&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&lt; ?=$former_search_txt?>&quot;+&quot;&amp;former_search_txt2=&lt; ?=$former_search_txt2?>&quot;+&quot;&amp;former_search_txt3=&lt; ?=$former_search_txt3?>&quot;);
            window.location.replace(&quot; , &quot;'&quot; , &quot;http://172.28.108.46:5454/smile/mod_ec/form/as1031.php?former_kategori_subkategori=Y&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&quot;+&quot;&amp;former_search_txt2=&quot;+&quot;&amp;sess=div-container-table&quot;+&quot;&amp;selected_data=&quot;);
	} 
		
	function emptyrows() {
            let html_data = &quot;&quot;;
            html_data += &quot; , &quot;'&quot; , &quot;&lt;tr class=&quot;nohover-color&quot;>&quot; , &quot;'&quot; , &quot;;
            html_data += &quot; , &quot;'&quot; , &quot;&lt;td colspan=&quot;10&quot; style=&quot;text-align: center;&quot;>-- Data tidak ditemukan --&lt;/td>&quot; , &quot;'&quot; , &quot;;
            html_data += &quot; , &quot;'&quot; , &quot;&lt;/tr>&quot; , &quot;'&quot; , &quot;;
            $(&quot;#data_list&quot;).html(html_data);
	}

	function download() {
            const linkDownload  = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            window.open(linkDownload, &quot; , &quot;'&quot; , &quot;_blank&quot; , &quot;'&quot; , &quot;).focus();
            //location.replace(&quot; , &quot;'&quot; , &quot;http://&lt; ?= $HTTP_HOST; ?>/mod_ec/form/as1031.php?former_kategori_subkategori=&lt; ?=$former_kategori_subkategori?>&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&lt; ?=$former_search_txt?>&quot;+&quot;&amp;former_search_txt2=&quot;+&quot;&amp;former_search_txt3=&lt; ?=$former_search_txt3?>&quot;);
            location.replace(&quot; , &quot;'&quot; , &quot;http://172.28.108.46:5454/smile/mod_ec/form/as1031.php?former_kategori_subkategori=Y&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&quot;+&quot;&amp;former_search_txt2=&quot;);
	}

	function regenerate(id_batch) {
            preloadMask(true);
            $.ajax({
                type: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;,
                url: &quot;http://172.28.108.46:5454/smile/mod_ec/ajax/as1031_action.php?&quot;+Math.random(),
                data: {
                    tipe: &quot; , &quot;'&quot; , &quot;regenerate&quot; , &quot;'&quot; , &quot;,
                    idBatch: id_batch
                },
                success: function(datas){
                    console.log(&quot;datas&quot;,datas);
                    jdata = JSON.parse(datas);
                    console.log(jdata);
                    if (jdata.ret == 1){
                        alert(jdata.msg);
                        //location.replace(&quot; , &quot;'&quot; , &quot;http://&lt; ?=$HTTP_HOST; ?>/mod_ec/form/as1031.php?former_kategori_subkategori=&lt; ?=$former_kategori_subkategori?>&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&lt; ?=$former_search_txt?>&quot;+&quot;&amp;former_search_txt2=&lt; ?=$former_search_txt2?>&quot;+&quot;&amp;former_search_txt3=&lt; ?=$former_search_txt3?>&quot;);
                        location.replace(&quot; , &quot;'&quot; , &quot;http://172.28.108.46:5454/smile/mod_ec/form/as1031.php?former_kategori_subkategori=Y&quot; , &quot;'&quot; , &quot;+&quot;&amp;former_search_txt=&quot;+&quot;&amp;former_search_txt2=&quot;+&quot;&amp;sess=div-container-table&quot;+&quot;&amp;selected_data=&quot;+&quot;&amp;status_crud=sukses&quot;);
                    } else {
                        alert(jdata.msg);
                    }
                    preloadMask(false);
                },
                complete: function(){
                    preloadMask(false);
                },
                error: function(){
                    alert(&quot;Terjadi kesalahan, coba beberapa saat lagi!&quot;);
                    preloadMask(false);
                }
            });
	}
        
        function addInputs(kode_konten, mode, obj) { //T = KATEGORI, Y = SUB KATEGORI
            /*$(&quot; , &quot;'&quot; , &quot;#konten_form&quot; , &quot;'&quot; , &quot;).clone().insertAfter(&quot; , &quot;'&quot; , &quot;div#konten2:last&quot; , &quot;'&quot; , &quot;);*/
            //$(&quot;div[id^=&quot; , &quot;'&quot; , &quot;konten_form&quot; , &quot;'&quot; , &quot;]:last&quot;).after($(&quot; , &quot;'&quot; , &quot;div#konten2&quot; , &quot;'&quot; , &quot;).clone());
            //alert(&quot; , &quot;'&quot; , &quot;test: &quot; , &quot;'&quot; , &quot;+kode_konten);
            
            //alert($(&quot;div#konten_form&quot;).length);
//            if (mode==&quot;change&quot;) {
//                //$(obj+&quot;.tipe_input,&quot;+obj+&quot;.kategori,&quot;+obj+&quot;.subkategori,&quot;+obj+&quot;.seq,&quot;+obj+&quot;.status&quot;).select2(&quot;destroy&quot;);
//                $(obj).remove();
//            }
            
            var no_konten_form = $(&quot;div#konten_form&quot;).length + 1;
            if (no_konten_form==11) {
                alert(&quot;Anda hanya bisa menambahkan 10 data.&quot;);
                return;
            }
            
            var no_urut = &quot;&quot;;
            for (let i = 1; i &lt;= 100; i++) {
                var selected = &quot;&quot;;
                if (no_konten_form==i) {
                    selected = &quot;selected=&quot; , &quot;'&quot; , &quot;selected&quot; , &quot;'&quot; , &quot;&quot;;
                }
                no_urut += &quot;&lt;option value=&quot; , &quot;'&quot; , &quot;&quot;+i+&quot;&quot; , &quot;'&quot; , &quot; &quot;+selected+&quot;>&quot;+i+&quot;&lt;/option>&quot;;
            }
            
            var input = &quot; , &quot;'&quot; , &quot;&lt;div id=&quot;konten_form&quot; class=&quot;konten_form&quot; , &quot;'&quot; , &quot;+no_konten_form+&quot; , &quot;'&quot; , &quot;&quot; style=&quot;float:left;position:relative;border-radius:10px;width:440px;height:128px;border:1px solid black;margin:10px;padding:40px 10px 10px 10px;&quot;>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div style=&quot;z-index:2;color:white;background: -webkit-linear-gradient(left,#6ba5ff,#416fd6);position:absolute;border-top-left-radius:10px;border-top-right-radius:10px;padding:6px 2px 2px 2px;font-weight:bold;font-size:12px;height:20px;left:0px;top:0px;right:0px;text-align:center;&quot;>Data Ke-&quot; , &quot;'&quot; , &quot;+no_konten_form+&quot; , &quot;'&quot; , &quot;&lt;img style=&quot;float:right;width:15px;height:auto;margin-right:4px;cursor:pointer;&quot; src=&quot;../../images/removex.png&quot; onclick=&quot;$(\&quot; , &quot;'&quot; , &quot;.konten_form&quot; , &quot;'&quot; , &quot;+no_konten_form+&quot; , &quot;'&quot; , &quot;\&quot; , &quot;'&quot; , &quot;).remove();&quot; />&lt;/div>&quot; , &quot;'&quot; , &quot;;
            if (kode_konten==&quot;T&quot; || kode_konten==&quot;KATEGORI&quot;) { //kategori
                    input += &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;form-row_kiri&quot;>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;label style= &quot;text-align:right;width: 130px;&quot;>Tipe &lt;span style=&quot;color:#ff0000;&quot;>*&lt;/span> :&lt;/label>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;select name=&quot;tipe_input[]&quot; class=&quot;tipe_input required&quot; style=&quot;width: 250px;&quot; accept=&quot;text/csv&quot; onchange=&quot;addInputs($(this).val(), \&quot; , &quot;'&quot; , &quot;change\&quot; , &quot;'&quot; , &quot;, \&quot; , &quot;'&quot; , &quot;.konten_form&quot; , &quot;'&quot; , &quot;+no_konten_form+&quot; , &quot;'&quot; , &quot;\&quot; , &quot;'&quot; , &quot;)&quot;>&quot; , &quot;'&quot; , &quot;+
                                            &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;KATEGORI&quot; selected=&quot;selected&quot;>KATEGORI&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                            //&quot; , &quot;'&quot; , &quot;&lt;option value=&quot;SUB KATEGORI&quot;>SUB KATEGORI&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;/select>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;clear&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;form-row_kiri&quot;>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;label style= &quot;text-align:right;width: 130px;&quot;>Nama Kategori &lt;span style=&quot;color:#ff0000;&quot;>*&lt;/span> :&lt;/label>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;input type=&quot;text&quot; name=&quot;kategori[]&quot; class=&quot;kategori required&quot; style=&quot;width: 245px;&quot; accept=&quot;text/csv&quot; required oninvalid=&quot;this.setCustomValidity(\&quot; , &quot;'&quot; , &quot;Input nama kategori.\&quot; , &quot;'&quot; , &quot;)&quot; oninput=&quot;this.setCustomValidity(\&quot; , &quot;'&quot; , &quot;\&quot; , &quot;'&quot; , &quot;); $(this).css(\&quot; , &quot;'&quot; , &quot;background-color\&quot; , &quot;'&quot; , &quot; , \&quot; , &quot;'&quot; , &quot;#FFFFFF\&quot; , &quot;'&quot; , &quot;);&quot; />&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;clear&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;form-row_kiri&quot;>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;label style= &quot;text-align:right;width: 130px;&quot;>No Urut &lt;span style=&quot;color:#ff0000;&quot;>*&lt;/span> :&lt;/label>&quot; , &quot;'&quot; , &quot;+
                                    //&quot; , &quot;'&quot; , &quot;&lt;select name=&quot;kategori[]&quot; id=&quot;kategori0&quot; class=&quot;kategori required&quot; style=&quot;width: 250px;display:none;&quot; accept=&quot;text/csv&quot;>&quot; , &quot;'&quot; , &quot;+
                                    //&quot; , &quot;'&quot; , &quot;&lt;/select>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;select name=&quot;seq[]&quot; class=&quot;seq required&quot; style=&quot;width: 250px;&quot; accept=&quot;text/csv&quot;>&quot; , &quot;'&quot; , &quot;+
                                        no_urut+
                                    &quot; , &quot;'&quot; , &quot;&lt;/select>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;clear&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;form-row_kiri&quot;>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;label style= &quot;text-align:right;width: 130px;&quot;>Status &lt;span style=&quot;color:#ff0000;&quot;>*&lt;/span> :&lt;/label>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;select name=&quot;status[]&quot; class=&quot;status required&quot; style=&quot;width: 250px;&quot; accept=&quot;text/csv&quot;>&quot; , &quot;'&quot; , &quot;+
                                            &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;N&quot; selected=&quot;selected&quot;>TAMBAH DATA KATEGORI&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                            &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;Y&quot; disabled=&quot;disabled&quot;>AKTIF&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                            &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;T&quot; disabled=&quot;disabled&quot;>NON AKTIF&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                            &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;SF0&quot; disabled=&quot;disabled&quot;>PENYESUAIAN NAMA KATEGORI&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                            &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;SF1&quot; disabled=&quot;disabled&quot;>PENYESUAIAN NOMOR URUT&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;/select>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;+
                        &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;;
                $(&quot;#konten2&quot;).append(input);
                $(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;).select2();
                //$(&quot; , &quot;'&quot; , &quot;select.tipe_input, select.kategori, select.seq, select.status&quot; , &quot;'&quot; , &quot;).select2();
                $(&quot; , &quot;'&quot; , &quot;select.tipe_input, select.seq, select.status&quot; , &quot;'&quot; , &quot;).select2();
            } else if (kode_konten==&quot;Y&quot; || kode_konten==&quot;SUB KATEGORI&quot;) { //sub kategori
                /*var no_urut2 = &quot;&quot;;
                for (let i = 1; i &lt;= 100; i++) {
                    no_urut2 += &quot;&lt;option value=&quot; , &quot;'&quot; , &quot;&quot;+i+&quot;&quot; , &quot;'&quot; , &quot;>KATEGORI &quot;+i+&quot;&lt;/option>&quot;;
                }
                //no_urut2 = $(&quot; , &quot;'&quot; , &quot;#konten_form.kategori:first&quot; , &quot;'&quot; , &quot;).html();*/
                //alert(no_urut2);
                
                    input += &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;form-row_kiri&quot;>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;label style= &quot;text-align:right;width: 130px;&quot;>Tipe &lt;span style=&quot;color:#ff0000;&quot;>*&lt;/span> :&lt;/label>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;select name=&quot;tipe_input[]&quot; class=&quot;tipe_input required&quot; style=&quot;width: 250px;&quot; accept=&quot;text/csv&quot; onchange=&quot;addInputs($(this).val(), \&quot; , &quot;'&quot; , &quot;change\&quot; , &quot;'&quot; , &quot;, \&quot; , &quot;'&quot; , &quot;.konten_form&quot; , &quot;'&quot; , &quot;+no_konten_form+&quot; , &quot;'&quot; , &quot;\&quot; , &quot;'&quot; , &quot;)&quot;>&quot; , &quot;'&quot; , &quot;+
                                            //&quot; , &quot;'&quot; , &quot;&lt;option value=&quot;KATEGORI&quot;>KATEGORI&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                            &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;SUB KATEGORI&quot; selected=&quot;selected&quot;>SUB KATEGORI&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;/select>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;clear&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;form-row_kiri&quot;>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;label style= &quot;text-align:right;width: 130px;&quot;>Nama Kategori &lt;span style=&quot;color:#ff0000;&quot;>*&lt;/span> :&lt;/label>&quot; , &quot;'&quot; , &quot;+
                                    //&quot; , &quot;'&quot; , &quot;&lt;select name=&quot;kategori[]&quot; class=&quot;kategori&quot; style=&quot;width: 250px;&quot; accept=&quot;text/csv&quot;>&lt;/select>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;div id=&quot;tool_placeholder_&quot; , &quot;'&quot; , &quot;+no_konten_form+&quot; , &quot;'&quot; , &quot;&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;clear&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;form-row_kiri&quot;>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;label style= &quot;text-align:right;width: 130px;&quot;>Nama Sub Kategori &lt;span style=&quot;color:#ff0000;&quot;>*&lt;/span> :&lt;/label>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;input type=&quot;text&quot; name=&quot;subkategori[]&quot; class=&quot;subkategori required&quot; style=&quot;width: 245px;&quot; accept=&quot;text/csv&quot; required oninvalid=&quot;this.setCustomValidity(\&quot; , &quot;'&quot; , &quot;Input nama sub kategori.\&quot; , &quot;'&quot; , &quot;)&quot; oninput=&quot;this.setCustomValidity(\&quot; , &quot;'&quot; , &quot;\&quot; , &quot;'&quot; , &quot;); $(this).css(\&quot; , &quot;'&quot; , &quot;background-color\&quot; , &quot;'&quot; , &quot; , \&quot; , &quot;'&quot; , &quot;#FFFFFF\&quot; , &quot;'&quot; , &quot;);&quot; />&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;clear&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;form-row_kiri&quot;>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;label style= &quot;text-align:right;width: 130px;&quot;>No Urut &lt;span style=&quot;color:#ff0000;&quot;>*&lt;/span> :&lt;/label>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;select name=&quot;seq[]&quot; class=&quot;seq required&quot; style=&quot;width: 250px;&quot; accept=&quot;text/csv&quot;>&quot; , &quot;'&quot; , &quot;+
                                        no_urut+
                                    &quot; , &quot;'&quot; , &quot;&lt;/select>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;clear&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;form-row_kiri&quot;>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;label style= &quot;text-align:right;width: 130px;&quot;>Status &lt;span style=&quot;color:#ff0000;&quot;>*&lt;/span> :&lt;/label>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;select name=&quot;status[]&quot; class=&quot;status required&quot; style=&quot;width: 250px;&quot; accept=&quot;text/csv&quot;>&quot; , &quot;'&quot; , &quot;+
                                            &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;N&quot; selected=&quot;selected&quot;>TAMBAH DATA SUB KATEGORI&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                            &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;Y&quot; disabled=&quot;disabled&quot;>AKTIF&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                            &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;T&quot; disabled=&quot;disabled&quot;>NON AKTIF&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                            &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;SF0&quot; disabled=&quot;disabled&quot;>PENYESUAIAN NAMA KATEGORI&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                            &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;SF0&quot; disabled=&quot;disabled&quot;>PENYESUAIAN NAMA SUB KATEGORI&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                            &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;SF1&quot; disabled=&quot;disabled&quot;>PENYESUAIAN NOMOR URUT&lt;/option>&quot; , &quot;'&quot; , &quot;+
                                    &quot; , &quot;'&quot; , &quot;&lt;/select>&quot; , &quot;'&quot; , &quot;+
                             &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;+
                        &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;;
                $(&quot;#konten2&quot;).append(input);
                
                //$(&quot; , &quot;'&quot; , &quot;select.kategori&quot; , &quot;'&quot; , &quot;).select2(&quot;destroy&quot;);
                var noOfSelect2 = $(&quot; , &quot;'&quot; , &quot;.kategori&quot; , &quot;'&quot; , &quot;).length;
                //var clonedSelect2 = $(&quot; , &quot;'&quot; , &quot;.kategori&quot; , &quot;'&quot; , &quot;).first().clone(true);
                var clonedSelect2 = $(&quot; , &quot;'&quot; , &quot;#kategori0&quot; , &quot;'&quot; , &quot;).clone(true);
                clonedSelect2.insertBefore(&quot;#tool_placeholder_&quot;+no_konten_form);
                clonedSelect2.attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;kategori&quot; , &quot;'&quot; , &quot;+noOfSelect2);
                clonedSelect2.attr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;kategori[]&quot; , &quot;'&quot; , &quot;);
                clonedSelect2.attr(&quot; , &quot;'&quot; , &quot;class&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;kategori&quot; , &quot;'&quot; , &quot;);
                clonedSelect2.attr(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;width: 250px;&quot; , &quot;'&quot; , &quot;);
                clonedSelect2.attr(&quot; , &quot;'&quot; , &quot;accept&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;text/csv&quot; , &quot;'&quot; , &quot;);
                $(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;).select2();
                //$(&quot; , &quot;'&quot; , &quot;select.kategori&quot; , &quot;'&quot; , &quot;).select2();
                $(&quot; , &quot;'&quot; , &quot;select#kategori&quot; , &quot;'&quot; , &quot;+noOfSelect2).select2();
                $(&quot; , &quot;'&quot; , &quot;select.tipe_input, select.kategori, select.seq, select.status&quot; , &quot;'&quot; , &quot;).select2();
            }
            
            $(&quot; , &quot;'&quot; , &quot;.kategori&quot; , &quot;'&quot; , &quot;).keypress(function (e) {
                console.log(&quot;keypress button: e.key=&quot;+e.key+&quot; - e.code=&quot;+e.code+&quot; - e.which=&quot;+e.which);
                /*if (e.which === 44 || e.key === &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&lt;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;>&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;?&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&quot;&quot; , &quot;'&quot; , &quot; || e.key === &quot;&quot; , &quot;'&quot; , &quot;&quot; || e.key === &quot; , &quot;'&quot; , &quot;{&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;[&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;}&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;|&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;\\&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;`&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;~&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;!&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;@&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;$&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;%&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;^&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;*&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;(&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;)&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;_&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;+&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;=&quot; , &quot;'&quot; , &quot;) {*/
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                    //alert(&quot;comma detected1&quot;);
                /*} else {
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replaceAll(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                }*/
            }).keyup(function (e) {
                /*if (e.which === 44 || e.key === &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&lt;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;>&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;?&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&quot;&quot; , &quot;'&quot; , &quot; || e.key === &quot;&quot; , &quot;'&quot; , &quot;&quot; || e.key === &quot; , &quot;'&quot; , &quot;{&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;[&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;}&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;|&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;\\&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;`&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;~&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;!&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;@&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;$&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;%&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;^&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;*&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;(&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;)&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;_&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;+&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;=&quot; , &quot;'&quot; , &quot;) {*/
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                    //alert(&quot;comma detected1&quot;);
                /*} else {
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replaceAll(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                }*/
            });
            
            $(&quot; , &quot;'&quot; , &quot;.subkategori&quot; , &quot;'&quot; , &quot;).keypress(function (e) {
                console.log(&quot;keypress button: e.key=&quot;+e.key+&quot; - e.code=&quot;+e.code+&quot; - e.which=&quot;+e.which);
                /*if (e.which === 44 || e.key === &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&lt;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;>&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;?&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&quot;&quot; , &quot;'&quot; , &quot; || e.key === &quot;&quot; , &quot;'&quot; , &quot;&quot; || e.key === &quot; , &quot;'&quot; , &quot;{&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;[&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;}&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;|&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;\\&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;`&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;~&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;!&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;@&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;$&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;%&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;^&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;*&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;(&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;)&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;_&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;+&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;=&quot; , &quot;'&quot; , &quot;) {*/
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                    //alert(&quot;comma detected1&quot;);
                /*} else {
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                }*/
            }).keyup(function (e) {
                /*if (e.which === 44 || e.key === &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&lt;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;>&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;?&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&quot;&quot; , &quot;'&quot; , &quot; || e.key === &quot;&quot; , &quot;'&quot; , &quot;&quot; || e.key === &quot; , &quot;'&quot; , &quot;{&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;[&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;}&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;|&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;\\&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;`&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;~&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;!&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;@&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;$&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;%&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;^&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;*&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;(&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;)&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;_&quot; , &quot;'&quot; , &quot; ||
                    e.key === &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;+&quot; , &quot;'&quot; , &quot; || e.key === &quot; , &quot;'&quot; , &quot;=&quot; , &quot;'&quot; , &quot;) {*/
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                    //alert(&quot;comma detected1&quot;);
                /*} else {
                    //$(this).val($(this).val().toUpperCase().replaceAll(&quot;,&quot;,&quot;&quot;));
                    $(this).val($(this).val().toUpperCase().replace(/[`~!@#$%^&amp;*()_|+\=?;:&quot; , &quot;'&quot; , &quot;&quot;,.&lt;>\{\}\[\]\\\/]/gi, &quot;&quot;));
                }*/
            });
        }


	.div-container{
		min-width: 700px;
		width: 100%;
	}
	.div-header{
		min-width: 700px;
	}
	.div-body{
		overflow-x: auto; 
		overflow-y: auto; 
		white-space: nowrap;
	}
	.div-data{
		overflow-x: auto; 
		overflow-y: auto; 
		white-space: nowrap;
	}
	.div-footer{
		padding-top: 10px;
		border-bottom: 1px solid #eeeeee;
	}
	.hr-double{
		border-top:3px double #8c8c8c;
		border-bottom:3px double #8c8c8c;
	}
  .hr-double-top{
    border-top:3px double #8c8c8c;
	}
  .hr-double-bottom{
  	border-bottom:3px double #8c8c8c;
	}
	.hr-double-left{
    border-left:3px double #8c8c8c;
	}
  .hr-double-right{
    border-right:3px double #8c8c8c;
	}
	.table-data{
		width: 100%;
		border-collapse: collapse;
		border-color: #c0c0c0;
		background-color: #ffffff;
	}
	.table-data th{
		padding: 10px 6px 10px 6px;
		font-weight: bold;
		text-align: left;
	}
	.table-data td{
		padding: 4px 6px 4px 6px;
		text-align: left;
		border-bottom: 1px solid #c0c0c0;
	}
	.table-data tr:last-child td{
		border-bottom:3px double #8c8c8c;
	}
	.table-data tbody tr:hover{
		cursor: pointer;
		background-color:#f5f5f5;
	}
  .nohover-color:hover {
		cursor: pointer!important;
    background-color:#FFFFFF!important;
	}
	.value-modified{
    background-color: #b4eeb4!important;
  }
  .l_frm{width: 180px; clear: left; float: left;margin-bottom: 2px;text-align: right; margin-right: 2px;}
    .r_frm{float: left;margin-bottom: 2px;}
    .r_frm input,.r_frm select {
        border-radius: 2px; 
        -moz-border-radius: 2px; 
        -webkit-border-radius: 2px; 
        border: 1px solid #bbb;
    }
    .column {
	  float: left;
	  padding: 1px;
	  /*height: 200px; /* Should be removed. Only for demonstration */
	}

	/* Clear floats after the columns */
	.row:after {
	  content: &quot;&quot;;
	  display: table;
	  clear: both;
	}

	input.button-accept {
	  width: 100%;
	  height:30px;
	  box-sizing: border-box;
	  border: 2px solid #ccc;
	  border-radius: 4px;
	  font-size: 18px;
	  font-weight: bold;
	  color:white;
	  background-color: #0091FF	;
	  /*background-image: url(../../images/app_form_edit.png);*/ 
	  background-image: url(../../images/accept.png);
	  background-position: 0px 0px; 
	  background-repeat: no-repeat;
	  padding: 0px 20px 12px 40px;
	}

	input.button-close {
	  width: 100%;
	  height:30px;
	  box-sizing: border-box;
	  border: 2px solid #ccc;
	  border-radius: 4px;
	  font-size: 18px;
	  font-weight: bold;
	  color:white;
	  background-color: #0091FF	;
	  background-image: url(../../images/cancel.png);
	  /*background-size: 20px;*/
	  background-position: 0px 0px; 
	  background-repeat: no-repeat;
	  padding: 0px 20px 12px 40px;
	}

	input.button-proses {
	  width: 100%;
	  height:30px;
	  box-sizing: border-box;
	  border: 2px solid #ccc;
	  border-radius: 4px;
	  font-size: 18px;
	  font-weight: bold;
	  color:white;
	  background-color: #0091FF	;
	  background-image: url(../../images/proses.png);
	  background-position: 0px 0px; 
	  background-repeat: no-repeat;
	  padding: 0px 20px 12px 40px;
	}

	input.button-verif {
	  width: 100%;
	  height:30px;
	  box-sizing: border-box;
	  border: 2px solid #ccc;
	  border-radius: 4px;
	  font-size: 18px;
	  font-weight: bold;
	  color:white;
	  background-color: #0091FF	;
	  background-image: url(../../images/open.gif);
	  background-position: 0px 0px; 
	  background-repeat: no-repeat;
	  padding: 0px 20px 12px 40px;
	}

	input.button-back {
	  width: 100%;
	  height:30px;
	  box-sizing: border-box;
	  border: 2px solid #ccc;
	  border-radius: 4px;
	  font-size: 18px;
	  font-weight: bold;
	  color:white;
	  background-color: #0091FF	;
	  background-image: url(../../images/application_view_columns.png); 
	  background-position: 0px 0px; 
	  background-repeat: no-repeat;
	  padding: 0px 20px 12px 40px;
	}




	AS1031 - CMS FAQ Kategori dan Sub Kategori [new] 


    
    
        
            
                
                
                
                    
                    
                    	
                        
                            
                                
                                    
                                    
                                        
                                            
                                            Detil Data					
                                             
                                             
                                             
                                            
                                                
                                                
                                                                                                     
                                                         Data Ke-1
                                                                                                                      
                                                                    Tipe * :
                                                                    
                                                                            
                                                                            SUB KATEGORI
                                                                    SUB KATEGORI
                                                              
                                                             
                                                             
                                                                    Nama Kategori * :
                                                                    
                                                                        FKAT001 - UMUMFKAT002 - PENERIMA UPAHFKAT003 - BUKAN PENERIMA UPAHFKAT004 - JAMINAN HARI TUA (JHT)FKAT005 - JAMINAN KECELAKAAN KERJA (JKK)FKAT006 - JAMINAN KEMATIAN (JKM)FKAT007 - JAMINAN PENSIUN (JP)FKAT008 - JAMINAN KEHILANGAN PEKERJAAN (JKP)FKAT009 - JASA KONSTRUKSIFKAT010 - LAYANAN SYARIAHFKAT011 - MANFAAT LAYANAN TAMBAHAN (MLT)FKAT012 - KANAL LAYANANFKAT013 - E-CHANNELFKAT014 - PEKERJA MIGRAN INDONESIA (PMI)FKAT015 - UJI KATALON 202412                                                                    FKAT015 - UJI KATALON 202412
                                                             		
                                                             
                                                             
                                                                    Nama Sub Kategori * :
                                                                    
                                                             		
                                                              
                                                             
                                                                    No Urut * :
                                                                    
                                                                            123456789101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899100                                                                    1
                                                              
                                                              
                                                             
                                                                    Status * :
                                                                    
                                                                            TAMBAH DATA SUB KATEGORI
                                                                            
                                                                    TAMBAH DATA SUB KATEGORI
                                                             
                                                                                                              
                                                                                                
                                            
                                        
                                    	
                                    
                                    
                                    
                                    
                                    
                                    
                                             

                                                                                		
                                            											
                                                    
                                                    
                                                    
                                            
                                     	
                                    
                                    
                                    
                                            Keterangan:
                                             Klik tombol TUTUP untuk kembali ke halaman utama
                                    
                                    
                                							
                            		
                        
                        
                        
                    
                	
            
        
    



 

  
    Loading...
  


/html[1]&quot;))]</value>
      <webElementGuid>056ff216-d89c-4296-a887-300f1e2834e7</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
