import React, { useState } from 'react';
import css from './navigation.less'
import { IconMoonFill, IconSunFill } from '@arco-design/web-react/icon';
import { Mode } from '~models/enum';
import { ModeLocalStorageKey } from '~common/common';

const setTheme = (mode: Mode) => {
  if (mode === Mode.Light) {
    document.body.removeAttribute('arco-theme');
  } else {
    document.body.setAttribute('arco-theme', 'dark');
  }
}

const Navigation: React.FC = () => {
  const modeOrigin = localStorage.getItem(ModeLocalStorageKey) === Mode.Dark ? Mode.Dark : Mode.Light;
  setTheme(modeOrigin)

  const [mode, setMode] = useState(modeOrigin);

  const changeMode = () => {
    let mode_ = mode === Mode.Dark ? Mode.Light : Mode.Dark;
    setTheme(mode_)
    localStorage.setItem(ModeLocalStorageKey, mode_)
    setMode(mode_);
  };

  return (
    <div className={css['navigation']}>
      <div className={css['nav-logo']}><a href='/'>记录 & 分享</a></div>
      <div className={css['nav-icon-list']}>
        <div className={css['icon']}>
          {mode === Mode.Light ? <IconSunFill onClick={changeMode} /> : <IconMoonFill onClick={changeMode} />}
        </div>
      </div>
    </div>
  );
};

export default Navigation;
