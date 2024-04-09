import React, { useState } from "react";
import QRCode from "react-qr-code";
import { Square } from "@chakra-ui/react";

interface QRCodeGeneratorProps {
  text: string;
}

const QRCodeGenerator: React.FC<QRCodeGeneratorProps> = ({ text }) => {
  return (
    <Square size="100px" borderColor={"white"} borderWidth={"2px"}>
      {text && (
        <QRCode
          value={text}
          // adjust the size as needed
        />
      )}
    </Square>
  );
};

export default QRCodeGenerator;
