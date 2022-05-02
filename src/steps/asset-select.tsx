import React, { FC } from 'react';

import * as Icon from '@geist-ui/react-icons';
import { Button, Row, Text } from '@geist-ui/react';

export const AssetSelect: FC<{
  loading: boolean;
  keys: string[];
  prodKey: string | null;
  assetFiles: string[];
  filesMissing: string[] | null;
  handleSetProdKey: (prodKey: string) => () => Promise<void>;
  handleSelectProdKey: () => Promise<void>;
  handleAddFiles: () => Promise<void>;
  handleRemoveFile: (fileName: string) => () => Promise<void>;
}> = ({
  loading,
  keys,
  prodKey,
  assetFiles,
  filesMissing,
  handleSetProdKey,
  handleSelectProdKey,
  handleAddFiles,
  handleRemoveFile
}) => {
  return (
    <>
      <Text>
        To decrypt your files (XCI or NSP), we need the prod.keys file from your
        Nintendo Switch. If you only want to unpack mods, it is not required.
      </Text>
      <div>Current prod.keys file: {prodKey ?? '-'}</div>
      <div
        style={{
          maxWidth: '36rem',
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
              style={prodKey === key ? { backgroundColor: 'darkgreen' } : {}}
              disabled={loading}
              iconRight={prodKey === key ? <Icon.Check /> : <Icon.Key />}
              onClick={prodKey !== key ? handleSetProdKey(key) : undefined}
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
      <div style={{ maxWidth: '36rem', minWidth: '24rem' }}>
        {assetFiles.map(assetFile => (
          <Row
            key={assetFile}
            style={{ marginBottom: '0.6rem' }}
            justify="space-between"
          >
            <span
              style={{
                flex: '1 1 auto',
                marginRight: '1rem',
                wordBreak: 'break-word'
              }}
            >
              {assetFile}
            </span>
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

      {filesMissing && (
        <div
          style={{
            maxWidth: '36rem',
            minWidth: '24rem',
            border: '2px solid #c4af0a',
            borderRadius: '8px',
            color: '#635801',
            padding: '0.8rem',
            margin: '0.6rem 0'
          }}
        >
          <Row align="middle">
            <Icon.AlertTriangle size={48} />
            <div style={{ width: '1.2rem' }} />
            <div style={{ fontWeight: 'bold', fontSize: '1.2rem' }}>
              Your assets are missing the following files in order to be able to
              play in Shroom Kingdom:
            </div>
          </Row>
          {filesMissing.map(file => (
            <Row
              key={file}
              style={{ marginBottom: '0.3rem' }}
              justify="space-between"
            >
              <span
                style={{
                  flex: '1 1 auto',
                  marginRight: '1rem',
                  wordBreak: 'break-word',
                  color: 'red',
                  fontSize: '0.8rem'
                }}
              >
                {file}
              </span>
            </Row>
          ))}
        </div>
      )}

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
