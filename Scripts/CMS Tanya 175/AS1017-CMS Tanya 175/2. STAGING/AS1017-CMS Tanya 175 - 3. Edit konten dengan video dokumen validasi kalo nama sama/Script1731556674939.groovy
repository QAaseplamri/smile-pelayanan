import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://smile-stg.bpjsketenagakerjaan.go.id/smile/login.bpjs')

WebUI.setText(findTestObject('Object Repository/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__login'), 'DH176590')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__password'), 
    'gKpkrQPjjYE=')

WebUI.click(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Login_button-1016-btnIconEl'))

WebUI.doubleClick(findTestObject('Object Repository/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__rule'))

WebUI.setText(findTestObject('Object Repository/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__rule'), 'PPE')

WebUI.click(findTestObject('Object Repository/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/div_PPE - Penata Pelayanan Elektronik ( 0 )'))

WebUI.click(findTestObject('Object Repository/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Pilih_button-1025-btnIconEl'))

WebUI.doubleClick(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_e-Channel'))

WebUI.doubleClick(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_JMO'))

WebUI.scrollToElement(findTestObject('Blasting Promo/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_AS1027-Upload Data Blasting Promo'), 
    0)

WebUI.click(findTestObject('Object Repository/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_AS1017- CMS Tanya 175'))

WebUI.setText(findTestObject('Object Repository/175/AS1017 - EDIT/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_search'), 
    'VIDEO TESTING')

WebUI.click(findTestObject('175/AS1017 - EDIT/AS1017 - EDIT_BUTTON'))

WebUI.setText(findTestObject('Object Repository/175/AS1017 - EDIT/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/textarea__nama_judul'), 
    'VIDEO TESTING 1')

WebUI.setText(findTestObject('Object Repository/175/AS1017 - EDIT/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/textarea__keterangan'), 
    'KET VIDEO TESTING 1')

WebUI.click(findTestObject('Object Repository/175/AS1017 - EDIT/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Save'))

WebUI.click(findTestObject('Object Repository/175/AS1017 - EDIT/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.click(findTestObject('Object Repository/175/AS1017 - EDIT/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.setText(findTestObject('Object Repository/175/AS1017 - EDIT/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_search'), 
    'DOKUMEN TESTING')

WebUI.click(findTestObject('175/AS1017 - EDIT/AS1017 - EDIT_BUTTON'))

WebUI.setText(findTestObject('Object Repository/175/AS1017 - EDIT/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/textarea__nama_judul'), 
    'DOKUMEN TESTING 1')

WebUI.setText(findTestObject('Object Repository/175/AS1017 - EDIT/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/textarea__keterangan_dokumen'), 
    'KET DOKUMEN TESTING 1')

WebUI.click(findTestObject('Object Repository/175/AS1017 - EDIT/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Save'))

WebUI.click(findTestObject('Object Repository/175/AS1017 - EDIT/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.click(findTestObject('Object Repository/175/AS1017 - EDIT/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.setText(findTestObject('Object Repository/175/AS1017 - EDIT/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_search'), 
    'MENGGUNAKAN VIDEO10')

WebUI.click(findTestObject('175/AS1017 - EDIT/AS1017 - EDIT_BUTTON'))

WebUI.setText(findTestObject('Object Repository/175/AS1017 - EDIT/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/textarea__nama_judul'), 
    'MENGGUNAKAN VIDEO70')

WebUI.setText(findTestObject('Object Repository/175/AS1017 - EDIT/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/textarea__keterangan'), 
    'MENGGUNAKAN VIDEO70')

WebUI.click(findTestObject('Object Repository/175/AS1017 - EDIT/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Save'))

WebUI.click(findTestObject('Object Repository/175/AS1017 - EDIT/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.click(findTestObject('Object Repository/175/AS1017 - EDIT/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.click(findTestObject('Object Repository/175/AS1017 - EDIT/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Close'))

WebUI.setText(findTestObject('Object Repository/175/AS1017 - EDIT/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_search'), 
    'MENGGUNAKAN DOKUMEN10')

WebUI.click(findTestObject('175/AS1017 - EDIT/AS1017 - EDIT_BUTTON'))

WebUI.setText(findTestObject('Object Repository/175/AS1017 - EDIT/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/textarea__nama_judul'), 
    'MENGGUNAKAN DOKUMEN70')

WebUI.setText(findTestObject('175/AS1017 - EDIT/AS1017 - FIELD KETERANGAN'), 'MENGGUNAKAN DOKUMEN70')

WebUI.click(findTestObject('Object Repository/175/AS1017 - EDIT/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Save'))

WebUI.click(findTestObject('Object Repository/175/AS1017 - EDIT/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.click(findTestObject('Object Repository/175/AS1017 - EDIT/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

