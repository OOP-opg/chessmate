module Game = struct
    external make_board : unit -> string = "" 
    [@@bs.module "chess-engine"] [@@bs.scope "Game"]
end
