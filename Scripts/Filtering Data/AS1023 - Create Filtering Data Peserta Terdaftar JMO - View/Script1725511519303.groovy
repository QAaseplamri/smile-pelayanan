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
import java.awt.Robot as Robot
import java.awt.event.KeyEvent as KeyEvent

WebUI.openBrowser('')

WebUI.maximizeWindow()

WebUI.navigateToUrl(GlobalVariable.Url_5331)

WebUI.setText(findTestObject('Object Repository/Filtering Data/Filtering Data/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__login'), 
    GlobalVariable.username)

WebUI.setText(findTestObject('Object Repository/Filtering Data/Filtering Data/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__password'), 
    GlobalVariable.password)

WebUI.click(findTestObject('Object Repository/Filtering Data/Filtering Data/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Login_button-1016-btnIconEl'))

WebUI.click(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'))

WebUI.clearText(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'))

'Input Role'
WebUI.setText(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'), 
    'PPE - Penata Pelayanan Elektronik ( 0 )')

WebUI.sendKeys(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'), 
    Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Object Repository/Filtering Data/Filtering Data/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Pilih_button-1025-btnIconEl'))

WebUI.doubleClick(findTestObject('Filtering Data/Filtering Data/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_e-Channel'))

WebUI.doubleClick(findTestObject('Filtering Data/Filtering Data/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_JMO'))

WebUI.click(findTestObject('Object Repository/Filtering Data/Filtering Data/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_AS1023-Filtering Data'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Filtering Data/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/select_-- Pilih --Nama Kriteria'), 
    'sc_nama_kriteria', true)

WebUI.click(findTestObject('Filtering Data/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/search_keyword'))

WebUI.setText(findTestObject('Filtering Data/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/search_keyword'), 'Peserta Baru 1')

WebUI.click(findTestObject('Filtering Data/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/btn_tampilkan_data'))

WebUI.takeScreenshot()

WebUI.click(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/b_View'))

WebUI.takeScreenshot()

