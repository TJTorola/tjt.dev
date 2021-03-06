import { createElement as h } from 'react';

import Random from './random.js';
import Stripes from './stripes.js';

export default {
  generators: {
    'depth-first': () => h('div', {}, 'Depth First: Not yet implimented'),
    'hilburts': () => h('div', {}, 'Hilburts: Not yet implimented'),
    'kruskals': () => h('div', {}, 'Kruscals: Not yet implimented'),
    'prims': () => h('div', {}, 'Prims: Not yet implimented'),
    'random': Random,
    'stripes': Stripes,
  },
  solvers: {
    'breadth-first': () => h('div', {}, 'Breadth First: Not yet implimented'),
    'dead-end-filling': () => h('div', {}, 'Dead End Filling: Not yet implimented'),
    'wall-follower': () => h('div', {}, 'Wall Follower: Not yet implimented'),
  }
}
