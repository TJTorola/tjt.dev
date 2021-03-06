import React from 'react';
import ReactDOM from 'react-dom';

import App from './app.jsx';
import * as serviceWorker from './serviceWorker.js';

ReactDOM.render(<App />, document.getElementById('root'));

serviceWorker.unregister();
