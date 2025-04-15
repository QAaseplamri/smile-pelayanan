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

WebUI.setText(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__login'), 
    'EK262740')

WebUI.setEncryptedText(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__password'), 
    'gKpkrQPjjYE=')

WebUI.delay(2)

WebUI.takeScreenshot()

boolean isVisible = WebUI.verifyElementVisible(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/buton_Yes'), 
    FailureHandling.OPTIONAL)

if (isVisible) {
    WebUI.click(findTestObject('Object Repository/New Folder/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

    WebUI.click(findTestObject('Object Repository/New Folder/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

    WebUI.click(findTestObject('Object Repository/New Folder/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Login_button-1016-btnIconEl'))
} else {
    WebUI.comment('Button Yes login tidak ditemukan!')

    WebUI.click(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Login_button-1016-btnIconEl'))
}

WebUI.doubleClick(findTestObject('Object Repository/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__rule'))

WebUI.setText(findTestObject('Object Repository/Konten faq/1 kategori/validasi-new-edt-delete/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__rule'), 
    'PPE')

WebUI.delay(2)

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/div_PPE - Penata Pelayanan Elektronik ( 0 )'))

WebUI.click(findTestObject('Object Repository/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Pilih_button-1025-btnIconEl'))

WebUI.doubleClick(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_e-Channel'))

WebUI.doubleClick(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_JMO'))

WebUI.scrollToElement(findTestObject('Konten faq/1 kategori/kategori-new-edit-delete-validasi/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_AS1031-CMS FAQ Kategori dan Sub Kategori'), 
    0)

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/validasi-new-edt-delete/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_AS1031-CMS FAQ Kategori dan Sub Kategori'))

WebUI.delay(2)

WebUI.takeScreenshot()

WebUI.delay(3)

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/validasi-new-edt-delete/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_'))

WebUI.delay(3)

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/validasi-new-edt-delete/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_Prev'))

WebUI.delay(3)

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/validasi-new-edt-delete/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_Next'))

WebUI.delay(3)

WebUI.click(findTestObject('Object Repository/Konten faq/1 kategori/validasi-new-edt-delete/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a__1'))

WebUI.delay(3)

WebUI.selectOptionByValue(findTestObject('Object Repository/Konten faq/1 kategori/validasi-new-edt-delete/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/select_1015202550'), 
    '50', true)

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_KATEGORI_rg_kategori'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_New'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_Sub Kategori'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_FKAT001 - UMUM'))

WebUI.setText(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_Loading_select2-search__field'), 
    'UJI KATALON 202412')

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_TAMBAH DATA SUB KATEGORI_btn_simpan'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.setText(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__subkategori'), 
    'SUB UJI KATALON 202412')

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_TAMBAH DATA SUB KATEGORI_btn_simpan'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_-- Pilih Kategori --'))

WebUI.setText(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_Loading_select2-search__field'), 
    'UJI KATALON_2024')

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_Status_id_subkategori'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_View'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_Edit'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_Delete'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_Status_id_subkategori'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_View'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_Status_id_subkategori'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_Edit'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_Status_id_subkategori'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_Edit'))

WebUI.setText(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__subkategori_1'), 
    'SUB UJI KATALON 202412 EDITEED')

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_TAMBAH DATA SUB KATEGORI_btn_simpan'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_Status_id_subkategori'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_Delete'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_New'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_Sub Kategori'))

WebUI.setText(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_Loading_select2-search__field'), 
    'UJI KATALON 202412')

WebUI.setText(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__subkategori'), 
    'SUB UJI KATALON 202412')

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_TAMBAH DATA SUB KATEGORI_btn_simpan'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.click(findTestObject('Object Repository/Konten faq/sub kategori fix/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.closeBrowser()

