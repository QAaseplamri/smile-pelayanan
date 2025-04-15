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
import com.kms.katalon.core.configuration.RunConfiguration as RunConfiguration

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

WebUI.click(findTestObject('Object Repository/175/AS1017 - NEW - VIDEO/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_New'))

WebUI.click(findTestObject('Object Repository/175/AS1017 - NEW - VIDEO/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Save'))

WebUI.click(findTestObject('Object Repository/175/AS1017 - NEW - VIDEO/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.takeScreenshot(RunConfiguration.getProjectDir() + '/Screenshoot/175/AS1017-judul-kosong-video.png')

WebUI.click(findTestObject('Object Repository/175/AS1017 - NEW - VIDEO/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.setText(findTestObject('Object Repository/175/AS1017 - NEW - VIDEO/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/textarea__nama_judul'), 
    'VIDEO TESTING')

WebUI.click(findTestObject('Object Repository/175/AS1017 - NEW - VIDEO/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Save'))

WebUI.click(findTestObject('Object Repository/175/AS1017 - NEW - VIDEO/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.takeScreenshot(RunConfiguration.getProjectDir() + '/Screenshoot/175/AS1017-kategori-kosong-video.png')

WebUI.click(findTestObject('Object Repository/175/AS1017 - NEW - VIDEO/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.click(findTestObject('Object Repository/175/AS1017 - NEW - VIDEO/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_-- Pilih Kategori --'))

WebUI.doubleClick(findTestObject('175/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_Loading_select2-search__field'), 
    FailureHandling.STOP_ON_FAILURE)

WebUI.setText(findTestObject('175/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_Loading_select2-search__field'), 
    'VIDEO DAN DOK TESTING 1')

WebUI.sendKeys(findTestObject('175/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_Loading_select2-search__field'), 
    Keys.chord(Keys.DOWN, Keys.DOWN, Keys.ENTER))

WebUI.click(findTestObject('Object Repository/175/AS1017 - NEW - VIDEO/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/div_Link Youtube Thumbnail Keterangan Dokum_be8ed9'))

WebUI.click(findTestObject('Object Repository/175/AS1017 - NEW - VIDEO/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Save'))

WebUI.click(findTestObject('Object Repository/175/AS1017 - NEW - VIDEO/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.takeScreenshot(RunConfiguration.getProjectDir() + '/Screenshoot/175/AS1017-keterangan-kosong-video.png')

WebUI.click(findTestObject('Object Repository/175/AS1017 - NEW - VIDEO/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.setText(findTestObject('Object Repository/175/AS1017 - NEW - VIDEO/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/textarea__url'), 
    'https://youtu.be/Hq9We4Jb44M')

WebUI.click(findTestObject('Object Repository/175/AS1017 - NEW - VIDEO/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/div_Link Youtube Thumbnail Keterangan Dokum_be8ed9'))

WebUI.click(findTestObject('Object Repository/175/AS1017 - NEW - VIDEO/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Save'))

WebUI.click(findTestObject('Object Repository/175/AS1017 - NEW - VIDEO/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.takeScreenshot(RunConfiguration.getProjectDir() + '/Screenshoot/175/AS1017-sukses-video.png')

WebUI.click(findTestObject('Object Repository/175/AS1017 - NEW - VIDEO/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.setText(findTestObject('Object Repository/175/AS1017 - NEW - VIDEO/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/textarea__keterangan'), 
    'KET VIDEO TESTING')

WebUI.click(findTestObject('Object Repository/175/AS1017 - NEW - VIDEO/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Save'))

WebUI.click(findTestObject('Object Repository/175/AS1017 - NEW - VIDEO/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.takeScreenshot(RunConfiguration.getProjectDir() + '/Screenshoot/175/AS1017-sukses-video.png')

WebUI.click(findTestObject('Object Repository/175/AS1017 - NEW - VIDEO/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

