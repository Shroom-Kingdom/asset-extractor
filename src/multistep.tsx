import React, { FC, ReactElement, useState } from 'react';

import { Button, Page, Row } from '@geist-ui/react';

export const MultiStep: FC<{
  steps: {
    component: ReactElement;
    backDisabled?: boolean;
    // eslint-disable-next-line @typescript-eslint/ban-types
    onNext?: Function;
    nextLabel?: string;
    nextIcon?: JSX.Element;
    nextDisabled?: boolean;
  }[];
}> = ({ steps }) => {
  const [currentStep, setCurrentStep] = useState(0);

  const navigateBackward = () => {
    setCurrentStep(Math.max(currentStep - 1, 0));
  };
  const navigateForward = () => {
    setCurrentStep(Math.min(currentStep + 1, steps.length));
    const onNext = steps[currentStep].onNext;
    if (onNext) onNext();
  };

  return (
    <Page>
      <Page.Content>{steps[currentStep].component}</Page.Content>
      <Page.Footer style={{ margin: '2rem 0' }}>
        <Row justify="space-around">
          <Button
            onClick={navigateBackward}
            style={{ visibility: currentStep === 0 ? 'hidden' : undefined }}
            type="secondary"
            disabled={steps[currentStep].backDisabled}
            ghost
          >
            Back
          </Button>
          <Button
            onClick={navigateForward}
            style={{
              visibility: currentStep + 1 >= steps.length ? 'hidden' : undefined
            }}
            type="success"
            iconRight={steps[currentStep].nextIcon ?? undefined}
            disabled={steps[currentStep].nextDisabled}
            ghost
          >
            {steps[currentStep].nextLabel ?? 'Next'}
          </Button>
        </Row>
      </Page.Footer>
    </Page>
  );
};
