import React from 'react';
import { useState, useEffect } from 'react'
import { useLocation } from 'react-router-dom';
import { marked } from 'marked';
import css from './blog.less';

const Blog = () => {
    const loc = useLocation();
    const content_: string | null = "";
    const [content, setContent] = useState(content_);
    useEffect(() => {
        fetch(`/api` + loc.pathname)
            .then((res) => res.text())
            .then((data: string) => {
                setContent(data);
            })
            .catch((err) => {
                console.log(err.message);
            });
    }, []);

    return <div className={css['blog']}>
        <div dangerouslySetInnerHTML={{ __html: marked.parse(content) }}></div>
    </div>
}

export default Blog;