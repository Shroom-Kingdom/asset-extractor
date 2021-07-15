import React from 'react';
import ReactDOM from 'react-dom';

import { CssBaseline, GeistProvider } from '@geist-ui/react';

import { App } from './app';

ReactDOM.render(
  <GeistProvider>
    <CssBaseline />
    <App />
  </GeistProvider>,
  document.getElementById('root')
);
