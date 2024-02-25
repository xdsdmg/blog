import React from 'react';
import { createRoot } from 'react-dom/client';
import "@arco-design/web-react/dist/css/arco.css";
import Home from '~pages/home/home';

const root = createRoot(document.getElementById('app') || document.body);

root.render(<Home></Home>);
