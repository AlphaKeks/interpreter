macro_rules! assert_token {
	(current, $parser:expr, $token:pat $(=> $ret:block)?) => {
		if let $token = &$parser.current_token {
			$( $ret )?
		} else {
			assert_token!(__error, $parser, $token);
		}
	};

	(peek, $parser:expr, $token:pat) => {{
		if let $token = &$parser.peek_token {
			$parser.step()?;
		} else {
			assert_token!(__error, $parser, $token);
		}
	}};

	(peek, $parser:expr, $token:pat => $ret:block) => {{
		if let $token = &$parser.peek_token {
			let ret = $ret;
			$parser.step()?;
			ret
		} else {
			assert_token!(__error, $parser, $token);
		}
	}};

	(__error, $parser:expr, $token:pat) => {
		::color_eyre::eyre::bail!(
			"Unexpected token. Got: {:?}, Wanted: {}",
			$parser.peek_token,
			stringify!($token),
		)
	};
}

pub(super) use assert_token;
