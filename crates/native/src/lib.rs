use pedersen;

use neon::prelude::*;

fn pedersen_starknet_crypto_from_hex(mut cx: FunctionContext) -> JsResult<JsString> {
    let x_js = cx.argument::<JsString>(0)?;
    let y_js = cx.argument::<JsString>(1)?;

    let result = pedersen::pedersen_starknet_crypto_from_hex(
        x_js.value(&mut cx).as_str(),
        y_js.value(&mut cx).as_str(),
    );

    Ok(cx.string(result))
}

fn pedersen_starknet_crypto_from_dec(mut cx: FunctionContext) -> JsResult<JsString> {
    let x_js = cx.argument::<JsString>(0)?;
    let y_js = cx.argument::<JsString>(1)?;

    let result = pedersen::pedersen_starknet_crypto_from_dec(
        x_js.value(&mut cx).as_str(),
        y_js.value(&mut cx).as_str(),
    );

    Ok(cx.string(result))
}

fn pedersen_geometry_from_hex(mut cx: FunctionContext) -> JsResult<JsString> {
    let x_js = cx.argument::<JsString>(0)?;
    let y_js = cx.argument::<JsString>(1)?;

    let result = pedersen_geometry::pedersen_geometry_from_hex(
        x_js.value(&mut cx).as_str(),
        y_js.value(&mut cx).as_str(),
    );

    Ok(cx.string(result))
}

fn pedersen_geometry_from_dec(mut cx: FunctionContext) -> JsResult<JsString> {
    let x_js = cx.argument::<JsString>(0)?;
    let y_js = cx.argument::<JsString>(1)?;

    let result = pedersen_geometry::pedersen_geometry_from_dec(
        x_js.value(&mut cx).as_str(),
        y_js.value(&mut cx).as_str(),
    );

    Ok(cx.string(result))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function(
        "pedersen_starknet_crypto_from_hex",
        pedersen_starknet_crypto_from_hex,
    )?;
    cx.export_function(
        "pedersen_starknet_crypto_from_dec",
        pedersen_starknet_crypto_from_dec,
    )?;

    cx.export_function("pedersen_geometry_from_hex", pedersen_geometry_from_hex)?;
    cx.export_function("pedersen_geometry_from_dec", pedersen_geometry_from_dec)?;

    Ok(())
}
