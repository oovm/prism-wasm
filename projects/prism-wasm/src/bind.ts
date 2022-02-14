import * as Prism from 'prismjs';

import 'prismjs/themes/prism.css'; /* or your own custom theme */
import 'prismjs/plugins/line-numbers/prism-line-numbers.css' /* add plugin css */

// Require all needed languages
require('prismjs/components/prism-javascript');
require('prismjs/components/prism-jsx');
require('prismjs/components/prism-typescript');
require('prismjs/components/prism-tsx');
require('prismjs/components/prism-css');
require('prismjs/components/prism-css-extras');
require('prismjs/components/prism-scss');
require('prismjs/components/prism-rust');

// Require all needed plugins
require('prismjs/plugins/line-numbers/prism-line-numbers');

export function prism_render(input: string, lang: string) {
    Prism.highlight(input, Prism.languages.js, lang)
}
