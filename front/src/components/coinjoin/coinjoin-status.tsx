import { Box, Button, Text, Link, useToast } from "@chakra-ui/react";

import { CoinJoinApi } from "@/apis";
import { FC, useEffect } from "react";
import moment from "moment";
import TimeAgo from "timeago-react";
import { useMutation, useQuery, useQueryClient } from "react-query";

import { listen } from "@tauri-apps/api/event";
import { CachePrefixKeys } from "@/consts";
import { useNoti } from "@/hooks";

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
  const noti = useNoti();
  const queryClient = useQueryClient();

  const { mutateAsync: onSignBtnClick, isLoading: isSigning } = useMutation(
    async (data: { deriv: string; roomId: string }) => {
      try {
        const res = await CoinJoinApi.signTxn(data.deriv, data.roomId);
        console.log("[CJ] Sign transaction response:", JSON.stringify(res));
        noti.success("Signed successfully");
        queryClient.invalidateQueries([
          CachePrefixKeys.RoomStatus,
          deriv,
          roomId,
        ]);
        queryClient.invalidateQueries([CachePrefixKeys.ListRooms]);
      } catch (e) {
        await CoinJoinApi.signTxn(data.deriv, data.roomId);
        noti.error("Got an error", e as string);
      }
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

  if (now < endOfDue1)
    return (
      <Box textAlign="right">
        <Text fontSize="16px" fontWeight="700" w="100%" color="yellow.100">
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
          color="cyan.200"
          fontWeight="500"
        >
          View on explorer
        </Link>
      </Box>
    );
  }

  if (now < endOfDue2) {
    if (signedQuery.data && signedQuery.data.status) {
      return (
        <Box textAlign="right">
          <Text fontSize="16px" fontWeight="700" w="100%" color="green.300">
            Signed successfully!
          </Text>
          <Text fontSize="14px" fontWeight="500" w="100%">
            Please wait for other peers...
          </Text>
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
      <Text fontSize="16px" fontWeight="700" w="100%" color="#faa">
        Failed!
      </Text>
      <Text fontSize="14px" fontWeight="500" w="100%" color="#aaa">
        Caused by missing signature(s)
      </Text>
    </Box>
  );
};
