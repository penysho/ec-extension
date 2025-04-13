async function initMocks() {
  if (process.env.NEXT_PUBLIC_API_MOCKING === "enabled") {
    console.log("🔧 MSW モックサーバーを初期化しています")

    if (typeof window === "undefined") {
      const { server } = await import("./server")
      server.listen()
      console.log("🔧 サーバーサイドモックが起動しました")
    } else {
      const { worker } = await import("./browser")
      worker.start()
      console.log("🔧 クライアントサイドモックが起動しました")
    }

    console.log("🔧 MSW モックサーバーの初期化が完了しました")
  }
}

initMocks()

export default initMocks
