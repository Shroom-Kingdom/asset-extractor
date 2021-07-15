import React, { FC, useState } from 'react';

import * as Icon from '@geist-ui/react-icons';
import {
  Button,
  Page,
  Progress,
  Row,
  Spacer,
  Text,
  Tooltip
} from '@geist-ui/react';

export const App: FC = () => {
  let gameFileInput: HTMLInputElement | null = null;
  const [gameFiles, setGameFiles] = useState<File[]>([]);

  const handleSelect = (event: React.ChangeEvent<HTMLInputElement>) => {
    if (!event.target.files) return;
    const file = event.target.files[0];
    if (!file) return;
    setGameFiles([file, ...gameFiles]);
  };

  const deleteFile = (fileName: string) => () => {
    const files = [...gameFiles];
    files.splice(
      files.findIndex(file => file.name === fileName),
      1
    );
    setGameFiles(files);
  };

  return (
    <>
      <Page>
        <Text>
          This is a simple tool to extract and bundle all assets that are
          required to play on Shroom Kingdom.
        </Text>
        <Spacer y={1} />
        <Text>
          Please select all your game resource files from Super Mario Maker 2:
        </Text>
        <div style={{ maxWidth: '24rem' }}>
          {gameFiles.map(gameFile => (
            <Row
              key={gameFile.name}
              style={{ marginBottom: '0.6rem' }}
              justify="space-between"
            >
              <span>{gameFile.name}</span>

              <Tooltip text={'Remove this entry'} type="dark">
                <Button
                  auto
                  size="mini"
                  icon={<Icon.Delete />}
                  onClick={deleteFile(gameFile.name)}
                />
              </Tooltip>
            </Row>
          ))}
        </div>
        <Button
          type="success-light"
          icon={<Icon.PlusCircle />}
          onClick={() => {
            if (gameFileInput) {
              gameFileInput.click();
            }
          }}
        >
          Add
        </Button>
        <input
          id="smm-game-file-input"
          type="file"
          accept=".xci,.nsp"
          multiple
          ref={ref => (gameFileInput = ref)}
          style={{ display: 'none' }}
          onChange={handleSelect}
        />
        <Spacer y={2} />
        <Progress value={50} />
      </Page>
    </>
  );
};
