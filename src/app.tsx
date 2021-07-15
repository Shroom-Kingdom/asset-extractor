import React, { FC } from 'react';

import { Button, Page, Spacer, Text } from '@geist-ui/react';

export const App: FC = () => {
  let gameFileInput: HTMLInputElement | null = null;

  const handleSelect = (event: React.ChangeEvent<HTMLInputElement>) => {
    if (!event.target.files) return;
    const file = event.target.files[0];
    if (!file) return;
    console.log('FILE', file);
  };

  return (
    <>
      <Page>
        <Text>
          This is a simple tool to extract and bundle all assets that are
          required to play on Shroom Kingdom.
        </Text>
        <Button
          type="success-light"
          onClick={() => {
            if (gameFileInput) {
              gameFileInput.click();
            }
          }}
        >
          Select
        </Button>
        <input
          id="smm-game-file-input"
          type="file"
          accept=".xci"
          ref={ref => (gameFileInput = ref)}
          style={{ display: 'none' }}
          onChange={handleSelect}
        />
        <Spacer y={2} />
      </Page>
    </>
  );
};
