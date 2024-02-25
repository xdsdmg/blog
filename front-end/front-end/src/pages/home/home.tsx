import React from 'react';
import Navigation from '~components/navigation/navigation';
import css from './home.less';
import { createBrowserRouter, RouterProvider, Navigate } from 'react-router-dom';
import Blog from '~pages/blog/blog';

const router = createBrowserRouter([
  {
    path: "/blog/*",
    element: <Blog />
  },
  {
    // Ref: https://stackoverflow.com/questions/69868956/how-can-i-redirect-in-react-router-v6
    path: "*",
    element: <Navigate to="/blog/hello.md" replace />
  }
]);

const Home: React.FC = () => {
  return (
    <div className={css['home']}>
      <Navigation />
      <div className={css['content']}>
        <div className={css['content-container']}>
          <React.StrictMode>
            <RouterProvider router={router}></RouterProvider>
          </React.StrictMode>
        </div>
      </div>
    </div>
  );
};

export default Home;
