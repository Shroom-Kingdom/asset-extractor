import React, { FC } from 'react';

import * as Icon from '@geist-ui/react-icons';
import { Button, Row, Text } from '@geist-ui/react';

export const AssetSelect: FC<{
  loading: boolean;
  keys: string[];
  prodKey: string | null;
  assetFiles: string[];
  handleSetProdKey: (prodKey: string) => () => Promise<void>;
  handleSelectProdKey: () => Promise<void>;
  handleAddFiles: () => Promise<void>;
  handleRemoveFile: (fileName: string) => () => Promise<void>;
}> = ({
  loading,
  keys,
  prodKey,
  assetFiles,
  handleSetProdKey,
  handleSelectProdKey,
  handleAddFiles,
  handleRemoveFile
}) => {
  return (
    <>
      <Text>
        To decrypt your files, we need the prod.keys file from your Nintendo
        Switch. If you only want to unpack mods, it is not required.
      </Text>
      <div>Current prod.keys file: {prodKey ?? '-'}</div>
      <div
        style={{
          maxWidth: '24rem',
          minWidth: '24rem'
        }}
      >
        {keys.map(key => (
          <Row key={key} style={{ margin: '0.4rem 0', alignItems: 'center' }}>
            <div
              style={{
                flex: '1 1 auto',
                marginRight: '1rem',
                wordBreak: 'break-word'
              }}
            >
              {key}
            </div>
            <Button
              auto
              size="mini"
              type="secondary-light"
              disabled={loading}
              iconRight={<Icon.Key />}
              onClick={handleSetProdKey(key)}
            >
              Select
            </Button>
          </Row>
        ))}
        <Button
          auto
          size="mini"
          type="secondary-light"
          disabled={loading}
          iconRight={<Icon.Key />}
          onClick={handleSelectProdKey}
        >
          {keys.length > 0 ? 'Manually select prod.keys' : 'Select prod.keys'}
        </Button>
      </div>
      <Text style={{ marginTop: '2rem' }}>
        Please select all your game resource files from Super Mario Maker 2:
      </Text>
      <div style={{ maxWidth: '24rem', minWidth: '24rem' }}>
        {assetFiles.map(assetFile => (
          <Row
            key={assetFile}
            style={{ marginBottom: '0.6rem' }}
            justify="space-between"
          >
            <span>{assetFile}</span>
            <Button
              auto
              size="mini"
              type="error"
              disabled={loading}
              icon={<Icon.Trash2 />}
              onClick={handleRemoveFile(assetFile)}
            />
          </Row>
        ))}
      </div>
      <Button
        type="success-light"
        disabled={loading}
        iconRight={<Icon.PlusCircle />}
        onClick={handleAddFiles}
      >
        Add
      </Button>
    </>
  );
};
