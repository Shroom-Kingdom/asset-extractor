import React, { FC } from 'react';

import * as Icon from '@geist-ui/react-icons';
import { Button, Row, Text } from '@geist-ui/react';

export const AssetSelect: FC<{
  loading: boolean;
  assetFiles: string[];
  handleAddFiles: () => Promise<void>;
  handleRemoveFile: (fileName: string) => () => Promise<void>;
}> = ({ loading, assetFiles, handleAddFiles, handleRemoveFile }) => {
  return (
    <>
      <Text>
        Please select all your game resource files from Super Mario Maker 2:
      </Text>
      <div style={{ maxWidth: '24rem' }}>
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
