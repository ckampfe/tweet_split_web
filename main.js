import init, { run_app } from './pkg/tweet_split_web.js';
async function main() {
   await init('/pkg/tweet_split_web_bg.wasm');
   run_app();
}
main()
