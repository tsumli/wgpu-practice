import init, { run } from '../wasm/wpr_render';

export default () => {
    init().then(() => {
        run()
    })
}