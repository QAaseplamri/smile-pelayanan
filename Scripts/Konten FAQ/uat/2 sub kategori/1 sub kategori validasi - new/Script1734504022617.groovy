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

WebUI.setText(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__login'), 
    'EK262740')

WebUI.setEncryptedText(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__password'), 
    'gKpkrQPjjYE=')

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

WebUI.click(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_AS1031-CMS FAQ Kategori dan Sub Kategori'))

WebUI.click(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_KATEGORI_rg_kategori'))

WebUI.click(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_New'))

WebUI.click(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_Sub Kategori'))

WebUI.setText(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_Loading_select2-search__field'), 
    'UJI COBA 09122024')

WebUI.setText(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__subkategori'), 
    'UJI COBA 09122024 SUBTEST')

WebUI.click(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_1'))

WebUI.click(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_TAMBAH DATA SUB KATEGORI_btn_simpan'))

WebUI.click(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.click(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.click(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_New'))

WebUI.click(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_Sub Kategori'))

WebUI.click(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_TAMBAH DATA SUB KATEGORI_btn_add_more'))

WebUI.click(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_TAMBAH DATA SUB KATEGORI_btn_add_more'))

WebUI.click(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_TAMBAH DATA SUB KATEGORI_btn_add_more'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/select_FKAT001 - UMUM EDITED 1 LAGI EDITED _660e09'), 
    'FKAT092', true)

WebUI.setText(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__subkategori'), 
    'UJI COBA 09122024 SUBTEST1')

WebUI.selectOptionByValue(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/select_FKAT001 - UMUM EDITED 1 LAGI EDITED _660e09'), 
    'FKAT001', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/select_FKAT001 - UMUM EDITED 1 LAGI EDITED _660e09'), 
    'FKAT092', true)

WebUI.setText(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__subkategori'), 
    'UJI COBA 09122024 SUBTEST2')

WebUI.selectOptionByValue(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/select_FKAT001 - UMUM EDITED 1 LAGI EDITED _660e09'), 
    'FKAT001', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/select_FKAT001 - UMUM EDITED 1 LAGI EDITED _660e09'), 
    'FKAT092', true)

WebUI.setText(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__subkategori'), 
    'UJI COBA 09122024 SUBTEST3')

WebUI.click(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_FKAT001 - UMUM EDITED 1 LAGI EDITED EDITED'))

WebUI.setText(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__subkategori'), 
    'UJI COBA 09122024 SUBTEST4')

WebUI.click(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_TAMBAH DATA SUB KATEGORI_btn_simpan'))

WebUI.click(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.click(findTestObject('Object Repository/Konten faq/2 sub kategori/1 sub kategori validasi - new/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.closeBrowser()

