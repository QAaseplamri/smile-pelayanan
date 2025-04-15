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

WebUI.navigateToUrl('http://172.28.108.46:5454/smile/login.bpjs')

WebUI.setText(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__login'), 
    'EK262740')

WebUI.setEncryptedText(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__password'), 
    GlobalVariable.password)

WebUI.click(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Login_button-1016-btnIconEl'))

WebUI.doubleClick(findTestObject('Object Repository/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__rule'))

WebUI.setText(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__rule'), 
    'PPE')

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/div_PPE - Penata Pelayanan Elektronik ( 0 )'))

WebUI.click(findTestObject('Object Repository/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Pilih_button-1025-btnIconEl'))

WebUI.doubleClick(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_e-Channel'))

WebUI.doubleClick(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_JMO'))

WebUI.scrollToElement(findTestObject('Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_AS1031-CMS FAQ Kategori dan Sub Kategori'), 
    0)

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_AS1031-CMS FAQ Kategori dan Sub Kategori'))

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_-- Pilih Kategori --'))

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_-- Pilih Sub Kategori --_btnfilter'))

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/div_View                                   _c16245'))

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_-- Pilih Sub Kategori --_btnfilter'))

WebUI.setText(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_-- Pilih Sub Kategori --_search_txt3'), 
    'BUKAN PENERIMA UPAH')

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_-- Pilih Sub Kategori --_btnfilter'))

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_-- Pilih Sub Kategori --_search_txt3'))

WebUI.doubleClick(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_-- Pilih Sub Kategori --_search_txt3'))

WebUI.setText(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_-- Pilih Sub Kategori --_search_txt3'), 
    '')

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_-- Pilih Sub Kategori --_btnfilter'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/select_1015202550'), 
    '50', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/select_1015202550'), 
    '10', true)

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_'))

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a__1'))

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_Next'))

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_Next'))

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_Prev'))

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_Prev'))

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_New'))

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_Kategori'))

WebUI.setText(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__kategori'), 
    '')

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_TAMBAH DATA KATEGORI_btn_simpan'))

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.setText(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__kategori'), 
    '20241218 UJI COBA KATALON')

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_TAMBAH DATA KATEGORI_btn_simpan'))

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_New'))

