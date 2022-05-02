/* eslint-disable react/jsx-no-target-blank */
import React, { FC } from 'react';

import { Text } from '@geist-ui/react';

export const Intro: FC = () => {
  return (
    <>
      <Text>
        This is a simple tool to extract and bundle all assets that are required
        to play on Shroom Kingdom.
      </Text>
      <Text>
        Please follow the{' '}
        <a href="https://yuzu-emu.org/help/quickstart/" target="_blank">
          Yuzu Quickstart Guide
        </a>{' '}
        to learn how to{' '}
        <a
          href="https://yuzu-emu.org/help/quickstart/#dumping-prodkeys-and-titlekeys"
          target="_blank"
        >
          dump your keys
        </a>{' '}
        and{' '}
        <a
          href="https://yuzu-emu.org/help/quickstart/#dumping-installed-titles-eshop"
          target="_blank"
        >
          game
        </a>
        .
      </Text>
      <Text>
        If you do not own the game, you can go to{' '}
        <a href="https://gamebanana.com/games/7348" target="_blank">
          Gamebanana
        </a>{' '}
        and download mods. You will most likely need several mods to get all
        assets that are required to play. Instead you can also download one of
        our preset: [WIP]
      </Text>
      <Text>
        This software is in early access. You can currently only extract XCI,
        ZIP and 7z files. Support for NSP files and game mods will be added
        later.
      </Text>
    </>
  );
};
