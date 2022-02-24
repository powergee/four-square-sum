import { useState, useEffect } from 'react';
import { Calculator } from '../components';
import ReactMarkdown from 'react-markdown';
import RemarkMathPlugin from 'remark-math';
import RehypeKatex from 'rehype-katex';
import introMd from '../shared/Introduction.md';
import "./Main.css"

function Main() {
  const [intro, setIntro] = useState("");

  useEffect(() => {
    fetch(introMd).then(res => res.text()).then(text => setIntro(text));
  }, []);

  return (
    <div className='main-root'>
      <Calculator></Calculator>
      <ReactMarkdown remarkPlugins={[RemarkMathPlugin]} rehypePlugins={[RehypeKatex]}>{intro}</ReactMarkdown>
    </div>
  );
}

export default Main;