use wasm_bindgen::prelude::*;

use cl;
use errors::IndyCryptoError;
use errors::ToErrorCode;
use serde;

impl From<IndyCryptoError> for JsValue {
    fn from(err: IndyCryptoError) -> JsValue {
        let error_code = err.to_error_code();
        JsValue::from_serde(&error_code).unwrap()
    }
}

fn convert_from_js<T>(val: &JsValue) -> Result<T, IndyCryptoError>
where
    for<'a> T: serde::Deserialize<'a>,
{
    match val.into_serde() {
        Ok(unwrapped) => Ok(unwrapped),
        Err(_) => Err(IndyCryptoError::InvalidStructure(
            "Invalid argument".to_string(),
        )),
    }
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn clNonce() -> Result<JsValue, JsValue> {
    let nonce = cl::new_nonce()?;
    Ok(JsValue::from_serde(&nonce).unwrap())
}
//CredentialSchemaBuilder
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn clCredentialSchemaBuilderNew() -> Result<JsValue, JsValue> {
    let builder = cl::CredentialSchemaBuilder::new()?;
    Ok(JsValue::from_serde(&builder).unwrap())
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn clCredentialSchemaBuilderAddAttr(cred_schem_builder: &JsValue, attr: &JsValue) -> Result<JsValue, JsValue> {
    let csb: cl::CredentialSchemaBuilder = convert_from_js(cred_schem_builder)?;
    let attr = convert_from_js(attr)?;
    Ok(JsValue::from_serde(csb.add_attr(&attr)).unwrap())// is this correct?
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn clCredentialSchemaBuilderFinalize(cred_schem_builder: &JsValue) -> Result<JsValue, JsValue> {
    let csb: cl::CredentialSchemaBuilder = convert_from_js(cred_schem_builder)?;
    Ok(JsValue::from_serde(csb.finalize()).unwrap())// is this correct?
}
//NonCredentialSchemaBuilder
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn clNonCredentialSchemaBuilderNew() -> Result<JsValue, JsValue> {
    let ncsb = cl::NonCredentialSchemaBuilder::new()?;
    Ok(JsValue::from_serde(ncsb).unwrap())
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn clNonCredentialSchemaBuilderAddAttr(non_cred_schem_builder: &JsValue, attr: &JsValue) -> Result<JsValue, JsValue> {
    let ncsb: cl::NonCredentialSchemaBuilder = convert_from_js(non_cred_schem_builder)?;
    let attr = convert_from_js(attr)?;
    Ok(JsValue::from_serde(ncsb.add_attr(&attr)).unwrap())// is this correct?
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn clNonCredentialSchemaBuilderFinalize(non_cred_schem_builder: &JsValue) -> Result<JsValue, JsValue> {
    let ncsb: cl::NonCredentialSchemaBuilder = convert_from_js(non_cred_schem_builder)?;
    Ok(JsValue::from_serde(ncsb.finalize()).unwrap())// is this correct?
}
//CredentialValue
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn clCredentialValueClone(cred_val: &JsValue) -> Result<JsValue, JsValue> {
    let cv: cl::CredentialValue = convert_from_js(cred_val)?;
    Ok(JsValue::from_serde(cv.clone()).unwrap())
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn clCredentialValueIsKnown(cred_val: &JsValue) -> Result<JsValue, JsValue> {
    let cv: cl::CredentialValue = convert_from_js(cred_val)?;
    Ok(JsValue::from_serde(cv.is_known()).unwrap())// is this correct? bool
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn clCredentialValueIsHidden(cred_val: &JsValue) -> Result<JsValue, JsValue> {
    let cv: cl::CredentialValue = convert_from_js(cred_val)?;
    Ok(JsValue::from_serde(cv.is_hidden()).unwrap())// is this correct? bool
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn clCredentialValueIsCommitment(cred_val: &JsValue) -> Result<JsValue, JsValue> {
    let cv: cl::CredentialValue = convert_from_js(cred_val)?;
    Ok(JsValue::from_serde(cv.is_commitment()).unwrap())// is this correct? bool
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn clCredentialValueIsCommitment(cred_val: &JsValue) -> Result<JsValue, JsValue> {
    let cv: cl::CredentialValue = convert_from_js(cred_val)?;
    Ok(JsValue::from_serde(cv.value()).unwrap())// is this correct? BigNumber?
}
//CredentialValues
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn clCredentialValuesClone(cred_val: &JsValue) -> Result<JsValue, JsValue> {
    let cv: cl::CredentialValues = convert_from_js(cred_val)?;
    Ok(JsValue::from_serde(cv.clone()).unwrap())
}
//CredentialValuesBuilder
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn clCredentialValuesBuilderNew() -> Result<JsValue, JsValue> {
    let cvb = cl::CredentialValuesBuilder::new()?;
    Ok(JsValue::from_serde(cvb).unwrap())
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn clCredentialValuesBuilderAddDecKnown(cred_val_builder: &JsValue, attr: &JsValue,value: &JsValue) -> Result<JsValue, JsValue> {
    let cvb: cl::CredentialValuesBuilder = convert_from_js(cred_val_builder)?;
    let attr = convert_from_js(attr)?;
    let val = convert_from_js(value)?;
    Ok(JsValue::from_serde(cvb.add_dec_known(&attr,&val)).unwrap())// is this correct? returns null?
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn clCredentialValuesBuilderAddDecHidden(cred_val_builder: &JsValue, attr: &JsValue,value: &JsValue) -> Result<JsValue, JsValue> {
    let cvb: cl::CredentialValuesBuilder = convert_from_js(cred_val_builder)?;
    let attr = convert_from_js(attr)?;
    let val = convert_from_js(value)?;
    Ok(JsValue::from_serde(cvb.add_dec_hidden(&attr,&val)).unwrap())// is this correct? returns null?
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn clCredentialValuesBuilderAddDecCommitment(cred_val_builder: &JsValue, attr: &JsValue,value: &JsValue, blinding_factor: &JsValue) -> Result<JsValue, JsValue> {
    let cvb: cl::CredentialValuesBuilder = convert_from_js(cred_val_builder)?;
    let attr = convert_from_js(attr)?;
    let val = convert_from_js(value)?;
    let bf = convert_from_js(blinding_factor)?;
    Ok(JsValue::from_serde(cvb.add_dec_commitment(&attr,&val,&bf)).unwrap())// is this correct? returns null?
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn clCredentialValuesBuilderAddValueKnown(cred_val_builder: &JsValue, attr: &JsValue,value: &JsValue) -> Result<JsValue, JsValue> {
    let cvb: cl::CredentialValuesBuilder = convert_from_js(cred_val_builder)?;
    let attr = convert_from_js(attr)?;
    let val = convert_from_js(value)?;
    Ok(JsValue::from_serde(cvb.add_value_known(&attr,&val)).unwrap())// is this correct? returns null?
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn clCredentialValuesBuilderAddValueHidden(cred_val_builder: &JsValue, attr: &JsValue,value: &JsValue) -> Result<JsValue, JsValue> {
    let cvb: cl::CredentialValuesBuilder = convert_from_js(cred_val_builder)?;
    let attr = convert_from_js(attr)?;
    let val = convert_from_js(value)?;// correct? bignum
    Ok(JsValue::from_serde(cvb.add_value_hidden(&attr,&val)).unwrap())// is this correct? returns null?
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn clCredentialValuesBuilderAddValueCommitment(cred_val_builder: &JsValue, attr: &JsValue,value: &JsValue, blinding_factor: &JsValue) -> Result<JsValue, JsValue> {
    let cvb: cl::CredentialValuesBuilder = convert_from_js(cred_val_builder)?;
    let attr = convert_from_js(attr)?;
    let val = convert_from_js(value)?;// correct? bignum
    let bf = convert_from_js(blinding_factor)?;// correct? bignum
    Ok(JsValue::from_serde(cvb.add_dec_commitment(&attr,&val,&bf)).unwrap())// is this correct? returns null?
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn clCredentialValuesBuilderFinalize(cred_val_builder: &JsValue) -> Result<JsValue, JsValue> {
    let cvb: cl::CredentialValuesBuilder = convert_from_js(cred_val_builder)?;
    Ok(JsValue::from_serde(cvb.finalize()).unwrap())// is this correct?
}
//CredentialPrimaryPublicKey
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn clCredentialPrimaryPublicKeyClone(cred_primary_pub_key: &JsValue) -> Result<JsValue, JsValue> {
    let cppk: cl::CredentialPrimaryPublicKey = convert_from_js(cred_primary_pub_key)?;
    Ok(JsValue::from_serde(cppk.clone()).unwrap())
}
//TODO: everything esle in cl

//....