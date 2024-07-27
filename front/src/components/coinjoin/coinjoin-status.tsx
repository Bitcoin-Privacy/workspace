import { Box, Button, Text, Link } from "@chakra-ui/react";

import { CoinJoinApi } from "@/apis";
import { FC, useEffect } from "react";
import moment from "moment";
import TimeAgo from "timeago-react";
import { useMutation, useQuery } from "react-query";

import { listen } from "@tauri-apps/api/event";
import { CachePrefixKeys } from "@/consts";

interface ICoinjoinStatus {
  deriv: string;
  roomId: string;
  status: number;
  txid: string | undefined;
  endOfDue1: number;
  endOfDue2: number;
}

export const CoinjoinStatus: FC<ICoinjoinStatus> = (props) => {
  const { deriv, roomId, endOfDue1, endOfDue2, status, txid } = props;
  const now = moment().unix() * 1000;

  const { mutateAsync: onSignBtnClick, isLoading: isSigning } = useMutation(
    async (data: { deriv: string; roomId: string }) => {
      await CoinJoinApi.signTxn(data.deriv, data.roomId);
    },
  );

  const signedQuery = useQuery(
    [CachePrefixKeys.RoomStatus, deriv, roomId],
    () => CoinJoinApi.getSigned(deriv, roomId),
  );

  useEffect(() => {
    console.log("startlisten");
    const unlisten = listen("coinjoin-setoutput", (event) => {
      console.log(2025117, "event", event);
    });

    return () => {
      unlisten
        .then((a) => a())
        .catch((e) => {
          console.log("UnListen Event failed:", e);
        });
    };
  }, []);

  // TODO: Check status of the room
  // Signed?
  // Completed?
  if (now < endOfDue1)
    return (
      <Box textAlign="right">
        <Text fontSize="16px" fontWeight="700" w="100%">
          Waiting for other peers...
        </Text>
        <Text fontSize="14px" fontWeight="500" w="100%">
          {"Move to the next step "}
          <TimeAgo datetime={endOfDue1} />
        </Text>
      </Box>
    );

  if (status === 3 && txid) {
    return (
      <Box textAlign="right">
        <Link
          href={`https://blockstream.info/testnet/tx/${txid}`}
          rel="noopener noreferrer"
          target="_blank"
        >
          View transaction on explorer
        </Link>
      </Box>
    );
  }

  if (now < endOfDue2) {
    if (signedQuery.data && signedQuery.data.status) {
      return (
        <Box textAlign="right">
          <Button
            isLoading={isSigning}
            isDisabled={isSigning}
            onClick={() => {
              onSignBtnClick({ deriv, roomId });
            }}
          >
            signed
          </Button>
        </Box>
      );
    }
    return (
      <Box textAlign="right">
        <Button
          isLoading={isSigning}
          isDisabled={isSigning}
          onClick={() => {
            onSignBtnClick({ deriv, roomId });
          }}
        >
          Sign
        </Button>
      </Box>
    );
  }

  return (
    <Box textAlign="right">
      <Text fontSize="16px" fontWeight="700" w="100%">
        Failed!
      </Text>
      <Text fontSize="14px" fontWeight="500" w="100%">
        caused by missing signature(s)
      </Text>
    </Box>
  );
};
